// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::types::*;

pub fn func_add_test_struct(_ctx: &ScFuncContext, _f: &AddTestStructContext) {
    let test_structs = _f.state.test_structs();
    let length = test_structs.length();
    let description: String = _f.params.description().value();
    let test_struct = TestStruct {
        id: length,
        description: description.clone()
    };
    test_structs.get_test_struct(length).set_value(&test_struct);

    _ctx.event(&format!(
        "teststruct.added {0} {1}",
        length.to_string(),
        description
    ))
}

pub fn func_clear_all(_ctx: &ScFuncContext, _f: &ClearAllContext) {
    let mut test_struct: Option<MutableTestStruct> = None;
    test_struct = Some(_f.state.test_structs().get_test_struct(0));


    _f.state.test_structs().clear();
    _ctx.log(&format!(
        "teststructs.cleared {0}",
        _f.state.test_structs().length().to_string(),
    ));

    for i in 0..10 { 
        let description = _f.state.test_structs().get_test_struct(i).value().description;
        let id = _f.state.test_structs().get_test_struct(i).value().id;
        _ctx.log(&format!(
            "teststructs.cleared.log {0} \n {1} {2} {3}",
            i.to_string(),
            _f.state.test_structs().length().to_string(),
            description,
            id.to_string()
        ));
    }

    test_struct.unwrap();
    _ctx.log(&format!(
        "teststruct.unwrapped {0} {1}",
        _f.state.test_structs().length().to_string(),
        _f.state.test_structs().get_test_struct(1).value().description
    ));

}