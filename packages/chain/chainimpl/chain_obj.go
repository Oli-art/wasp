// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package chainimpl

import (
	"bytes"
	txstream "github.com/iotaledger/goshimmer/packages/txstream/client"
	"github.com/iotaledger/hive.go/events"
	"github.com/iotaledger/hive.go/logger"
	"github.com/iotaledger/wasp/packages/chain"
	"github.com/iotaledger/wasp/packages/chain/consensus"
	"github.com/iotaledger/wasp/packages/chain/mempool"
	"github.com/iotaledger/wasp/packages/chain/statemgr"
	"github.com/iotaledger/wasp/packages/coretypes"
	"github.com/iotaledger/wasp/packages/hashing"
	"github.com/iotaledger/wasp/packages/peering"
	"github.com/iotaledger/wasp/packages/registry"
	"github.com/iotaledger/wasp/packages/tcrypto"
	"github.com/iotaledger/wasp/packages/vm/processors"
	"go.uber.org/atomic"
	"golang.org/x/xerrors"
	"sync"
)

type chainObj struct {
	committee             chain.Committee
	mempool               chain.Mempool
	dismissed             atomic.Bool
	dismissOnce           sync.Once
	chainID               coretypes.ChainID
	procset               *processors.ProcessorCache
	chMsg                 chan interface{}
	stateMgr              chain.StateManager
	consensus             chain.Consensus
	eventRequestProcessed *events.Event
	log                   *logger.Logger
	nodeConn              *txstream.Client
	netProvider           peering.NetworkProvider
	dksProvider           tcrypto.RegistryProvider
	blobProvider          coretypes.BlobCache
}

func NewChain(
	chr *registry.ChainRecord,
	log *logger.Logger,
	nodeConn *txstream.Client,
	netProvider peering.NetworkProvider,
	dksProvider tcrypto.RegistryProvider,
	blobProvider coretypes.BlobCache,
) chain.Chain {
	log.Debugf("creating chain object for %s", chr.ChainID.String())

	chainLog := log.Named(chr.ChainID.Base58()[:6] + ".")
	ret := &chainObj{
		mempool: mempool.New(blobProvider, chainLog),
		procset: processors.MustNew(),
		chMsg:   make(chan interface{}, 100),
		chainID: chr.ChainID,
		eventRequestProcessed: events.NewEvent(func(handler interface{}, params ...interface{}) {
			handler.(func(_ coretypes.RequestID))(params[0].(coretypes.RequestID))
		}),
		log:          chainLog,
		nodeConn:     nodeConn,
		netProvider:  netProvider,
		dksProvider:  dksProvider,
		blobProvider: blobProvider,
	}
	ret.stateMgr = statemgr.New(ret, newPeers(nil), nodeConn, ret.log)
	go func() {
		for msg := range ret.chMsg {
			ret.dispatchMessage(msg)
		}
	}()
	ret.startTimer()
	return ret
}

// iAmInTheCommittee checks if NetIDs makes sense
func iAmInTheCommittee(committeeNodes []string, n, index uint16, netProvider peering.NetworkProvider) bool {
	if len(committeeNodes) != int(n) {
		return false
	}
	return committeeNodes[index] == netProvider.Self().NetID()
}

func (c *chainObj) dispatchMessage(msg interface{}) {
	switch msgt := msg.(type) {
	case *peering.PeerMessage:
		c.processPeerMessage(msgt)
	case *chain.StateUpdateMsg:
		c.stateMgr.EventStateUpdateMsg(msgt)
	case *chain.StateTransitionMsg:
		if c.consensus != nil {
			c.consensus.EventStateTransitionMsg(msgt)
		}
	case chain.BlockCandidateMsg:
		c.stateMgr.EventBlockCandidateMsg(msgt)
	case *chain.InclusionStateMsg:
		if c.consensus != nil {
			c.consensus.EventTransactionInclusionStateMsg(msgt)
		}
	case *chain.StateMsg:
		c.processStateMessage(msgt)
	case *chain.VMResultMsg:
		// VM finished working
		if c.consensus != nil {
			c.consensus.EventResultCalculated(msgt)
		}

	case chain.TimerTick:
		if msgt%2 == 0 {
			c.stateMgr.EventTimerMsg(msgt / 2)
		} else {
			if c.consensus != nil {
				c.consensus.EventTimerMsg(msgt / 2)
			}
		}
		if msgt%40 == 0 {
			total, withMsg, solid := c.mempool.Stats()
			c.log.Debugf("mempool total = %d, withMsg = %d solid = %d", total, withMsg, solid)
		}
	}
}

func (c *chainObj) processPeerMessage(msg *peering.PeerMessage) {
	rdr := bytes.NewReader(msg.MsgData)

	switch msg.MsgType {

	case chain.MsgStateIndexPingPong:
		msgt := &chain.BlockIndexPingPongMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}
		msgt.SenderIndex = msg.SenderIndex

		c.stateMgr.EventStateIndexPingPongMsg(msgt)

	case chain.MsgNotifyRequests:
		msgt := &chain.NotifyReqMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex

		if c.consensus != nil {
			c.consensus.EventNotifyReqMsg(msgt)
		}

	case chain.MsgNotifyFinalResultPosted:
		msgt := &chain.NotifyFinalResultPostedMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex

		if c.consensus != nil {
			c.consensus.EventNotifyFinalResultPostedMsg(msgt)
		}

	case chain.MsgStartProcessingRequest:
		msgt := &chain.StartProcessingBatchMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex
		msgt.Timestamp = msg.Timestamp

		if c.consensus != nil {
			c.consensus.EventStartProcessingBatchMsg(msgt)
		}

	case chain.MsgSignedHash:
		msgt := &chain.SignedHashMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex
		msgt.Timestamp = msg.Timestamp

		if c.consensus != nil {
			c.consensus.EventSignedHashMsg(msgt)
		}

	case chain.MsgGetBatch:
		msgt := &chain.GetBlockMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex

		c.stateMgr.EventGetBlockMsg(msgt)

	case chain.MsgBatchHeader:
		msgt := &chain.BlockHeaderMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex
		c.stateMgr.EventBlockHeaderMsg(msgt)

	case chain.MsgStateUpdate:
		msgt := &chain.StateUpdateMsg{}
		if err := msgt.Read(rdr); err != nil {
			c.log.Error(err)
			return
		}

		msgt.SenderIndex = msg.SenderIndex
		c.stateMgr.EventStateUpdateMsg(msgt)

	default:
		c.log.Errorf("processPeerMessage: wrong msg type")
	}
}

// processStateMessage processes chain output
// If necessary, it creates/changes committee object and sends new peers to the stateManager
func (c *chainObj) processStateMessage(msg *chain.StateMsg) {
	sh, err := hashing.HashValueFromBytes(msg.ChainOutput.GetStateData())
	if err != nil {
		c.log.Error(xerrors.Errorf("parsing state hash: %w", err))
		return
	}
	c.log.Debugw("processStateMessage",
		"stateIndex", msg.ChainOutput.GetStateIndex(),
		"stateHash", sh.String(),
		"stateAddr", msg.ChainOutput.GetStateAddress().Base58(),
	)
	if c.committee != nil && c.committee.DKShare().Address.Equals(msg.ChainOutput.GetStateAddress()) {
		// nothing changed in the committee
		c.stateMgr.EventStateMsg(msg)
		return
	}
	// create or change committee object
	if c.committee != nil {
		c.committee.Close()
	}
	if c.consensus != nil {
		c.consensus.Close()
	}
	c.committee, c.consensus = nil, nil
	c.log.Debugf("creating new committee...")
	if c.committee, err = NewCommittee(c, msg.ChainOutput.GetStateAddress(), c.netProvider, c.dksProvider, c.log); err != nil {
		c.committee = nil
		c.log.Errorf("failed to create committee object for address %s: %v", msg.ChainOutput.GetStateAddress(), err)
		return
	}
	c.committee.OnPeerMessage(func(recv *peering.RecvEvent) {
		c.ReceiveMessage(recv.Msg)
	})
	c.log.Debugf("created new committee for state address %s", msg.ChainOutput.GetStateAddress().Base58())

	c.log.Debugf("creating new consensus object..")
	c.consensus = consensus.New(c.mempool, c.committee, c.nodeConn, c.log)
	c.stateMgr.SetPeers(newPeers(c.committee))
	c.stateMgr.EventStateMsg(msg)
}
