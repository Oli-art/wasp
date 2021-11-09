package test

import (
	"testing"

	"github.com/iotaledger/wasp/test"
	"github.com/iotaledger/wasp/packages/vm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, test.ScName, test.OnLoad)
	require.NoError(t, ctx.ContractExists(test.ScName))
}
