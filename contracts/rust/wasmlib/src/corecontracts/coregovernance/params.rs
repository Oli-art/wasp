// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::corecontracts::coregovernance::*;
use crate::host::*;

#[derive(Clone, Copy)]
pub struct ImmutableAddAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl ImmutableAddAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableAddAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl MutableAddAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableRemoveAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl ImmutableRemoveAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableRemoveAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl MutableRemoveAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableRotateStateControllerParams {
    pub(crate) id: i32,
}

impl ImmutableRotateStateControllerParams {
    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableRotateStateControllerParams {
    pub(crate) id: i32,
}

impl MutableRotateStateControllerParams {
    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}