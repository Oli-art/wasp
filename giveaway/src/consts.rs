// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "giveaway";
pub const SC_DESCRIPTION : &str = "giveaway smart contract to choose the winners of the Zentangle Drangon's giveaways. Addresses should not be repeated";
pub const HSC_NAME       : ScHname = ScHname(0x31774d34);

pub const PARAM_ADDRESSES : &str = "addresses";
pub const PARAM_N_WINNERS : &str = "nWinners";
pub const PARAM_OWNER     : &str = "owner";

pub const RESULT_OWNER   : &str = "owner";
pub const RESULT_WINNERS : &str = "winners";

pub const STATE_ADDRESSES : &str = "addresses";
pub const STATE_OWNER     : &str = "owner";

pub const FUNC_INIT             : &str = "init";
pub const FUNC_LOAD_ADDRESSES   : &str = "loadAddresses";
pub const FUNC_RUFFLE           : &str = "ruffle";
pub const FUNC_SET_OWNER        : &str = "setOwner";
pub const FUNC_UNLOAD_ADDRESSES : &str = "unloadAddresses";
pub const VIEW_GET_OWNER        : &str = "getOwner";

pub const HFUNC_INIT             : ScHname = ScHname(0x1f44d644);
pub const HFUNC_LOAD_ADDRESSES   : ScHname = ScHname(0x2e3febf1);
pub const HFUNC_RUFFLE           : ScHname = ScHname(0x1a23d876);
pub const HFUNC_SET_OWNER        : ScHname = ScHname(0x2a15fe7b);
pub const HFUNC_UNLOAD_ADDRESSES : ScHname = ScHname(0xb8992203);
pub const HVIEW_GET_OWNER        : ScHname = ScHname(0x137107a6);