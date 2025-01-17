// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub(crate) const IDX_PARAM_DESCRIPTION:             usize = 0;
pub(crate) const IDX_PARAM_H:                       usize = 1;
pub(crate) const IDX_PARAM_IMAGE_ID:                usize = 2;
pub(crate) const IDX_PARAM_NUMBER_OF_IMAGES:        usize = 3;
pub(crate) const IDX_PARAM_TAGS_REQUIRED_PER_IMAGE: usize = 4;
pub(crate) const IDX_PARAM_W:                       usize = 5;
pub(crate) const IDX_PARAM_X:                       usize = 6;
pub(crate) const IDX_PARAM_Y:                       usize = 7;
pub(crate) const IDX_RESULT_H:                      usize = 8;
pub(crate) const IDX_RESULT_IMAGE_ID:               usize = 9;
pub(crate) const IDX_RESULT_PLAYS_PER_IMAGE:        usize = 10;
pub(crate) const IDX_RESULT_W:                      usize = 11;
pub(crate) const IDX_RESULT_X:                      usize = 12;
pub(crate) const IDX_RESULT_Y:                      usize = 13;
pub(crate) const IDX_STATE_BETS:                    usize = 14;
pub(crate) const IDX_STATE_CREATOR:                 usize = 15;
pub(crate) const IDX_STATE_DESCRIPTION:             usize = 16;
pub(crate) const IDX_STATE_NUMBER_OF_IMAGES:        usize = 17;
pub(crate) const IDX_STATE_PENDING_PLAYS:           usize = 18;
pub(crate) const IDX_STATE_PLAYS_PER_IMAGE:         usize = 19;
pub(crate) const IDX_STATE_PROCESSED_IMAGES:        usize = 20;
pub(crate) const IDX_STATE_REWARD:                  usize = 21;
pub(crate) const IDX_STATE_TAGGED_IMAGES:           usize = 22;
pub(crate) const IDX_STATE_TAGS_REQUIRED_PER_IMAGE: usize = 23;

pub const KEY_MAP_LEN: usize = 24;

pub const KEY_MAP: [&str; KEY_MAP_LEN] = [
    PARAM_DESCRIPTION,
    PARAM_H,
    PARAM_IMAGE_ID,
    PARAM_NUMBER_OF_IMAGES,
    PARAM_TAGS_REQUIRED_PER_IMAGE,
    PARAM_W,
    PARAM_X,
    PARAM_Y,
    RESULT_H,
    RESULT_IMAGE_ID,
    RESULT_PLAYS_PER_IMAGE,
    RESULT_W,
    RESULT_X,
    RESULT_Y,
    STATE_BETS,
    STATE_CREATOR,
    STATE_DESCRIPTION,
    STATE_NUMBER_OF_IMAGES,
    STATE_PENDING_PLAYS,
    STATE_PLAYS_PER_IMAGE,
    STATE_PROCESSED_IMAGES,
    STATE_REWARD,
    STATE_TAGGED_IMAGES,
    STATE_TAGS_REQUIRED_PER_IMAGE,
];

pub static mut IDX_MAP: [Key32; KEY_MAP_LEN] = [Key32(0); KEY_MAP_LEN];

pub fn idx_map(idx: usize) -> Key32 {
    unsafe {
        IDX_MAP[idx]
    }
}

// @formatter:on
