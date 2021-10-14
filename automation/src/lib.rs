// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

#![allow(unused_imports)]

use automation::*;
use wasmlib::*;
use wasmlib::host::*;

use crate::consts::*;
use crate::keys::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;

mod consts;
mod contract;
mod keys;
mod params;
mod results;
mod state;
mod types;
mod automation;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func(FUNC_INIT, func_init_thunk);
    exports.add_func(FUNC_ISSUER_CONFIRM_RESULT, func_issuer_confirm_result_thunk);
    exports.add_func(FUNC_ISSUER_REJECT_RESULT, func_issuer_reject_result_thunk);
    exports.add_func(FUNC_ISSUER_REQUEST_TASK, func_issuer_request_task_thunk);
    exports.add_func(FUNC_MACHINE_FINNISH_TASK, func_machine_finnish_task_thunk);
    exports.add_func(FUNC_MACHINE_QUIT_TASK, func_machine_quit_task_thunk);
    exports.add_func(FUNC_MACHINE_RESPONSE, func_machine_response_thunk);
    exports.add_view(VIEW_GET_OWNER, view_get_owner_thunk);
    exports.add_view(VIEW_GET_TASK, view_get_task_thunk);

    unsafe {
        for i in 0..KEY_MAP_LEN {
            IDX_MAP[i] = get_key_id_from_string(KEY_MAP[i]);
        }
    }
}

pub struct InitContext {
    params: ImmutableInitParams,
    state:  MutableautomationState,
}

fn func_init_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcInit");
    let f = InitContext {
        params: ImmutableInitParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    func_init(ctx, &f);
    ctx.log("automation.funcInit ok");
}

pub struct IssuerConfirmResultContext {
    params: ImmutableIssuerConfirmResultParams,
    state:  MutableautomationState,
}

fn func_issuer_confirm_result_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcIssuerConfirmResult");
    let f = IssuerConfirmResultContext {
        params: ImmutableIssuerConfirmResultParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    func_issuer_confirm_result(ctx, &f);
    ctx.log("automation.funcIssuerConfirmResult ok");
}

pub struct IssuerRejectResultContext {
    params: ImmutableIssuerRejectResultParams,
    state:  MutableautomationState,
}

fn func_issuer_reject_result_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcIssuerRejectResult");
    let f = IssuerRejectResultContext {
        params: ImmutableIssuerRejectResultParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    func_issuer_reject_result(ctx, &f);
    ctx.log("automation.funcIssuerRejectResult ok");
}

pub struct IssuerRequestTaskContext {
    params: ImmutableIssuerRequestTaskParams,
    state:  MutableautomationState,
}

fn func_issuer_request_task_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcIssuerRequestTask");
    let f = IssuerRequestTaskContext {
        params: ImmutableIssuerRequestTaskParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.instruction().exists(), "missing mandatory instruction");
    ctx.require(f.params.machine_id().exists(), "missing mandatory machineId");
    func_issuer_request_task(ctx, &f);
    ctx.log("automation.funcIssuerRequestTask ok");
}

pub struct MachineFinnishTaskContext {
    params: ImmutableMachineFinnishTaskParams,
    state:  MutableautomationState,
}

fn func_machine_finnish_task_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcMachineFinnishTask");
    let f = MachineFinnishTaskContext {
        params: ImmutableMachineFinnishTaskParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    func_machine_finnish_task(ctx, &f);
    ctx.log("automation.funcMachineFinnishTask ok");
}

pub struct MachineQuitTaskContext {
    params: ImmutableMachineQuitTaskParams,
    state:  MutableautomationState,
}

fn func_machine_quit_task_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcMachineQuitTask");
    let f = MachineQuitTaskContext {
        params: ImmutableMachineQuitTaskParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    func_machine_quit_task(ctx, &f);
    ctx.log("automation.funcMachineQuitTask ok");
}

pub struct MachineResponseContext {
    params: ImmutableMachineResponseParams,
    state:  MutableautomationState,
}

fn func_machine_response_thunk(ctx: &ScFuncContext) {
    ctx.log("automation.funcMachineResponse");
    let f = MachineResponseContext {
        params: ImmutableMachineResponseParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.response().exists(), "missing mandatory response");
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    func_machine_response(ctx, &f);
    ctx.log("automation.funcMachineResponse ok");
}

pub struct GetOwnerContext {
    results: MutableGetOwnerResults,
    state:   ImmutableautomationState,
}

fn view_get_owner_thunk(ctx: &ScViewContext) {
    ctx.log("automation.viewGetOwner");
    let f = GetOwnerContext {
        results: MutableGetOwnerResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    view_get_owner(ctx, &f);
    ctx.log("automation.viewGetOwner ok");
}

pub struct GetTaskContext {
    params:  ImmutableGetTaskParams,
    results: MutableGetTaskResults,
    state:   ImmutableautomationState,
}

fn view_get_task_thunk(ctx: &ScViewContext) {
    ctx.log("automation.viewGetTask");
    let f = GetTaskContext {
        params: ImmutableGetTaskParams {
            id: OBJ_ID_PARAMS,
        },
        results: MutableGetTaskResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableautomationState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.task_id().exists(), "missing mandatory taskId");
    view_get_task(ctx, &f);
    ctx.log("automation.viewGetTask ok");
}

// @formatter:on
