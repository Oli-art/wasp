// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::types::*;

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

pub fn func_machine_finnish_task(_ctx: &ScFuncContext, _f: &MachineFinnishTaskContext) {
}

pub fn func_machine_response(_ctx: &ScFuncContext, _f: &MachineResponseContext) {
}

pub fn func_request_machine(_ctx: &ScFuncContext, _f: &RequestMachineContext) {
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}
