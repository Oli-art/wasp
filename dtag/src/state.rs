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
use crate::types::*;

pub struct ArrayOfImmutableBet {
    pub(crate) obj_id: i32,
}

impl ArrayOfImmutableBet {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_bet(&self, index: i32) -> ImmutableBet {
        ImmutableBet { obj_id: self.obj_id, key_id: Key32(index) }
    }
}

pub struct ArrayOfImmutableInt32 {
    pub(crate) obj_id: i32,
}

impl ArrayOfImmutableInt32 {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_int32(&self, index: i32) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.obj_id, Key32(index))
    }
}

pub struct ArrayOfImmutableTaggedImage {
    pub(crate) obj_id: i32,
}

impl ArrayOfImmutableTaggedImage {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_tagged_image(&self, index: i32) -> ImmutableTaggedImage {
        ImmutableTaggedImage { obj_id: self.obj_id, key_id: Key32(index) }
    }
}

#[derive(Clone, Copy)]
pub struct ImmutabledtagState {
    pub(crate) id: i32,
}

impl ImmutabledtagState {
    pub fn bets(&self) -> ArrayOfImmutableBet {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_BETS), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfImmutableBet { obj_id: arr_id }
    }

    pub fn creator(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.id, idx_map(IDX_STATE_CREATOR))
    }

    pub fn description(&self) -> ScImmutableString {
        ScImmutableString::new(self.id, idx_map(IDX_STATE_DESCRIPTION))
    }

    pub fn number_of_images(&self) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.id, idx_map(IDX_STATE_NUMBER_OF_IMAGES))
    }

    pub fn pending_plays(&self) -> ArrayOfImmutableBet {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAYS), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfImmutableBet { obj_id: arr_id }
    }

    pub fn plays_per_image(&self) -> ArrayOfImmutableInt32 {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYS_PER_IMAGE), TYPE_ARRAY | TYPE_INT32);
        ArrayOfImmutableInt32 { obj_id: arr_id }
    }

    pub fn processed_images(&self) -> ArrayOfImmutableTaggedImage {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PROCESSED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfImmutableTaggedImage { obj_id: arr_id }
    }

    pub fn reward(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, idx_map(IDX_STATE_REWARD))
    }

    pub fn tagged_images(&self) -> ArrayOfImmutableTaggedImage {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_TAGGED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfImmutableTaggedImage { obj_id: arr_id }
    }

    pub fn tags_required_per_image(&self) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.id, idx_map(IDX_STATE_TAGS_REQUIRED_PER_IMAGE))
    }
}

pub struct ArrayOfMutableBet {
    pub(crate) obj_id: i32,
}

impl ArrayOfMutableBet {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_bet(&self, index: i32) -> MutableBet {
        MutableBet { obj_id: self.obj_id, key_id: Key32(index) }
    }
}

pub struct ArrayOfMutableInt32 {
    pub(crate) obj_id: i32,
}

impl ArrayOfMutableInt32 {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_int32(&self, index: i32) -> ScMutableInt32 {
        ScMutableInt32::new(self.obj_id, Key32(index))
    }
}

pub struct ArrayOfMutableTaggedImage {
    pub(crate) obj_id: i32,
}

impl ArrayOfMutableTaggedImage {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_tagged_image(&self, index: i32) -> MutableTaggedImage {
        MutableTaggedImage { obj_id: self.obj_id, key_id: Key32(index) }
    }
}

#[derive(Clone, Copy)]
pub struct MutabledtagState {
    pub(crate) id: i32,
}

impl MutabledtagState {
    pub fn bets(&self) -> ArrayOfMutableBet {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_BETS), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfMutableBet { obj_id: arr_id }
    }

    pub fn creator(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.id, idx_map(IDX_STATE_CREATOR))
    }

    pub fn description(&self) -> ScMutableString {
        ScMutableString::new(self.id, idx_map(IDX_STATE_DESCRIPTION))
    }

    pub fn number_of_images(&self) -> ScMutableInt32 {
        ScMutableInt32::new(self.id, idx_map(IDX_STATE_NUMBER_OF_IMAGES))
    }

    pub fn pending_plays(&self) -> ArrayOfMutableBet {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAYS), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfMutableBet { obj_id: arr_id }
    }

    pub fn plays_per_image(&self) -> ArrayOfMutableInt32 {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYS_PER_IMAGE), TYPE_ARRAY | TYPE_INT32);
        ArrayOfMutableInt32 { obj_id: arr_id }
    }

    pub fn processed_images(&self) -> ArrayOfMutableTaggedImage {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PROCESSED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfMutableTaggedImage { obj_id: arr_id }
    }

    pub fn reward(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, idx_map(IDX_STATE_REWARD))
    }

    pub fn tagged_images(&self) -> ArrayOfMutableTaggedImage {
        let arr_id = get_object_id(self.id, idx_map(IDX_STATE_TAGGED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
        ArrayOfMutableTaggedImage { obj_id: arr_id }
    }

    pub fn tags_required_per_image(&self) -> ScMutableInt32 {
        ScMutableInt32::new(self.id, idx_map(IDX_STATE_TAGS_REQUIRED_PER_IMAGE))
    }
}
