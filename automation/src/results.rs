// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

use crate::*;
use crate::keys::*;

#[derive(Clone, Copy)]
pub struct ImmutableGetOwnerResults {
    pub(crate) id: i32,
}

impl ImmutableGetOwnerResults {
    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.id, idx_map(IDX_RESULT_OWNER))
    }
}

#[derive(Clone, Copy)]
pub struct MutableGetOwnerResults {
    pub(crate) id: i32,
}

impl MutableGetOwnerResults {
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.id, idx_map(IDX_RESULT_OWNER))
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableGetTasksResults {
    pub(crate) id: i32,
}

impl ImmutableGetTasksResults {
    pub fn task(&self) -> ScImmutableTask {
        ScImmutableTask::new(self.id, idx_map(IDX_RESULT_TASK))
    }
}

#[derive(Clone, Copy)]
pub struct MutableGetTasksResults {
    pub(crate) id: i32,
}

impl MutableGetTasksResults {
    pub fn task(&self) -> ScMutableTask {
        ScMutableTask::new(self.id, idx_map(IDX_RESULT_TASK))
    }
}
