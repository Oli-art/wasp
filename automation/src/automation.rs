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

pub fn func_issuer_confirm_result(_ctx: &ScFuncContext, _f: &IssuerConfirmResultContext) {
}

pub fn func_issuer_reject_result(_ctx: &ScFuncContext, _f: &IssuerRejectResultContext) {
}

pub fn func_issuer_request_task(_ctx: &ScFuncContext, _f: &IssuerRequestTaskContext) {

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();

    // Retrieve the amount of plain iota tokens that are part of the incoming balance.
    let amount: i64 = incoming.balance(&ScColor::IOTA);

    // Now we gather all information together into a single serializable struct
    let task = Task {
        machine_id: _f.params.machine_id().value(),
        issuer: _ctx.caller(),
        instruction: _f.params.instruction().value(),
        amount: amount,
        status: "requested".to_string()
    };

    // Get the array of tasks from state storage.
    let tasks: ArrayOfMutableTask = _f.state.tasks();

    // Determine what the next bet number is by retrieving the length of the bets array.
    let task_nr: i32 = tasks.length();

    // Append the bet data to the bets array. The bet array will automatically take care
    // of serializing the bet struct into a bytes representation.
    tasks.get_task(task_nr).set_value(&task);

    _ctx.event(&format!(
        "task.requested {0} {1} {2} {3}",
        task_nr.to_string(),
        &task.issuer.to_string(),
        task.instruction,
        task.amount.to_string()
    ));
}

pub fn func_machine_finnish_task(_ctx: &ScFuncContext, _f: &MachineFinnishTaskContext) {
}

pub fn func_machine_quit_task(_ctx: &ScFuncContext, _f: &MachineQuitTaskContext) {
}

pub fn func_machine_response(_ctx: &ScFuncContext, _f: &MachineResponseContext) {

    // Retrieve the id of the task and the response: 0 for reject and 1 for approve
    let task_id = _f.params.task_id().value();
    let response =_f.params.response().value();

    let task = _f.state.tasks().get_task(task_id);

    _ctx.require(task.value().machine_id == _ctx.caller(), "Responded to a task assigned to another machine_id");
    

    let _task = Task{
        machine_id: task.value().machine_id,
        issuer: task.value().issuer,
        instruction: task.value().instruction,
        amount: task.value().amount,
        status: "rejected".to_string()
    };

    if response == 1 {
        _task = Task{
            machine_id: task.value().machine_id,
            issuer: task.value().issuer,
            instruction: task.value().instruction,
            amount: task.value().amount,
            status: "accepted".to_string()
        };
    }
    
    //let tasks = _f.state.tasks();

    //tasks[task_id] = _task;
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_task(_ctx: &ScViewContext, _f: &GetTaskContext) {
    let tasks: ArrayOfImmutableTask = _f.state.tasks();
    let task_id = _f.params.task_id().value();

    _f.results.task().set_value(tasks.get_task(task_id));
}
