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

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();

    // Retrieve the amount of plain iota tokens that are part of the incoming balance.
    let amount: i64 = incoming.balance(&ScColor::IOTA);

    // Now we gather all information together into a single serializable struct
    let task = Task {
        machine_id: _f.params.machine_id().value(),
        issuer: _ctx.caller(),
        task:&_f.params.task().value(),
        amount: amount
    };

    // Get the array of tasks from state storage.
    let tasks: ArrayOfMutableTask = _f.state.tasks();

    // Determine what the next bet number is by retrieving the length of the bets array.
    let task_nr: i32 = tasks.length();

    // Append the bet data to the bets array. The bet array will automatically take care
    // of serializing the bet struct into a bytes representation.
    tasks.get_task(task_nr).set_value(&task);

    _ctx.event(&format!(
        "machine.requested {0} {1} {2}",
        &task.issuer.to_string(),
        task.task,
        task.amount
    ));
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_tasks(_ctx: &ScViewContext, _f: &GetTasksContext) {
    // Get the array of tasks from state storage.
    let tasks: ArrayOfImmutableTask = _f.state.tasks();

    _f.results.task().set_value(tasks);
}
