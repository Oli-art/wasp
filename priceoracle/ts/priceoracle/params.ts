// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export class ImmutableApproveOracleParams extends wasmtypes.ScProxy {
	agentid(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamAgentid));
	}
}

export class MutableApproveOracleParams extends wasmtypes.ScProxy {
	agentid(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamAgentid));
	}
}

export class ImmutableInitParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class MutableInitParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class ImmutableSetMiotaPriceParams extends wasmtypes.ScProxy {
	price(): wasmtypes.ScImmutableInt64 {
		return new wasmtypes.ScImmutableInt64(this.proxy.root(sc.ParamPrice));
	}
}

export class MutableSetMiotaPriceParams extends wasmtypes.ScProxy {
	price(): wasmtypes.ScMutableInt64 {
		return new wasmtypes.ScMutableInt64(this.proxy.root(sc.ParamPrice));
	}
}

export class ImmutableSetOwnerParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class MutableSetOwnerParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}
