// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use zentangle::*;
use wasmlib::*;

use crate::consts::*;
use crate::events::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;
use crate::structs::*;

mod consts;
mod contract;
mod events;
mod params;
mod results;
mod state;
mod structs;
mod utility_funcs;
mod zentangle;

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
    	FUNC_CREATE_GAME,
    	FUNC_END_GAME,
    	FUNC_INIT,
    	FUNC_REQUEST_PLAY,
    	FUNC_SEND_TAGS,
    	FUNC_SET_OWNER,
    	FUNC_WITHDRAW,
    	VIEW_GET_OWNER,
    	VIEW_GET_PLAYER_BETS,
    	VIEW_GET_PLAYER_INFO,
    	VIEW_GET_PLAYS_PER_IMAGE,
    	VIEW_GET_RESULTS,
	],
    funcs: &[
    	func_create_game_thunk,
    	func_end_game_thunk,
    	func_init_thunk,
    	func_request_play_thunk,
    	func_send_tags_thunk,
    	func_set_owner_thunk,
    	func_withdraw_thunk,
	],
    views: &[
    	view_get_owner_thunk,
    	view_get_player_bets_thunk,
    	view_get_player_info_thunk,
    	view_get_plays_per_image_thunk,
    	view_get_results_thunk,
	],
};

#[no_mangle]
fn on_call(index: i32) {
	ScExports::call(index, &EXPORT_MAP);
}

#[no_mangle]
fn on_load() {
    ScExports::export(&EXPORT_MAP);
}

pub struct CreateGameContext {
	events:  zentangleEvents,
	params: ImmutableCreateGameParams,
	state: MutablezentangleState,
}

fn func_create_game_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcCreateGame");
	let f = CreateGameContext {
		events:  zentangleEvents {},
		params: ImmutableCreateGameParams { proxy: params_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};
	ctx.require(f.params.description().exists(), "missing mandatory description");
	ctx.require(f.params.number_of_images().exists(), "missing mandatory numberOfImages");
	func_create_game(ctx, &f);
	ctx.log("zentangle.funcCreateGame ok");
}

pub struct EndGameContext {
	events:  zentangleEvents,
	params: ImmutableEndGameParams,
	state: MutablezentangleState,
}

fn func_end_game_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcEndGame");
	let f = EndGameContext {
		events:  zentangleEvents {},
		params: ImmutableEndGameParams { proxy: params_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};
	func_end_game(ctx, &f);
	ctx.log("zentangle.funcEndGame ok");
}

pub struct InitContext {
	events:  zentangleEvents,
	params: ImmutableInitParams,
	state: MutablezentangleState,
}

fn func_init_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcInit");
	let f = InitContext {
		events:  zentangleEvents {},
		params: ImmutableInitParams { proxy: params_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};
	func_init(ctx, &f);
	ctx.log("zentangle.funcInit ok");
}

pub struct RequestPlayContext {
	events:  zentangleEvents,
	results: MutableRequestPlayResults,
	state: MutablezentangleState,
}

fn func_request_play_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcRequestPlay");
	let f = RequestPlayContext {
		events:  zentangleEvents {},
		results: MutableRequestPlayResults { proxy: results_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};
	func_request_play(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.funcRequestPlay ok");
}

pub struct SendTagsContext {
	events:  zentangleEvents,
	params: ImmutableSendTagsParams,
	results: MutableSendTagsResults,
	state: MutablezentangleState,
}

fn func_send_tags_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcSendTags");
	let f = SendTagsContext {
		events:  zentangleEvents {},
		params: ImmutableSendTagsParams { proxy: params_proxy() },
		results: MutableSendTagsResults { proxy: results_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};
	ctx.require(f.params.input_json().exists(), "missing mandatory inputJson");
	func_send_tags(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.funcSendTags ok");
}

pub struct SetOwnerContext {
	events:  zentangleEvents,
	params: ImmutableSetOwnerParams,
	state: MutablezentangleState,
}

fn func_set_owner_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcSetOwner");
	let f = SetOwnerContext {
		events:  zentangleEvents {},
		params: ImmutableSetOwnerParams { proxy: params_proxy() },
		state: MutablezentangleState { proxy: state_proxy() },
	};

	// current owner of this smart contract
	let access = f.state.owner();
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller() == access.value(), "no permission");

	ctx.require(f.params.owner().exists(), "missing mandatory owner");
	func_set_owner(ctx, &f);
	ctx.log("zentangle.funcSetOwner ok");
}

pub struct WithdrawContext {
	events:  zentangleEvents,
	state: MutablezentangleState,
}

fn func_withdraw_thunk(ctx: &ScFuncContext) {
	ctx.log("zentangle.funcWithdraw");
	let f = WithdrawContext {
		events:  zentangleEvents {},
		state: MutablezentangleState { proxy: state_proxy() },
	};

	// current owner of this smart contract
	let access = f.state.owner();
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller() == access.value(), "no permission");

	func_withdraw(ctx, &f);
	ctx.log("zentangle.funcWithdraw ok");
}

pub struct GetOwnerContext {
	results: MutableGetOwnerResults,
	state: ImmutablezentangleState,
}

fn view_get_owner_thunk(ctx: &ScViewContext) {
	ctx.log("zentangle.viewGetOwner");
	let f = GetOwnerContext {
		results: MutableGetOwnerResults { proxy: results_proxy() },
		state: ImmutablezentangleState { proxy: state_proxy() },
	};
	view_get_owner(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.viewGetOwner ok");
}

pub struct GetPlayerBetsContext {
	results: MutableGetPlayerBetsResults,
	state: ImmutablezentangleState,
}

fn view_get_player_bets_thunk(ctx: &ScViewContext) {
	ctx.log("zentangle.viewGetPlayerBets");
	let f = GetPlayerBetsContext {
		results: MutableGetPlayerBetsResults { proxy: results_proxy() },
		state: ImmutablezentangleState { proxy: state_proxy() },
	};
	view_get_player_bets(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.viewGetPlayerBets ok");
}

pub struct GetPlayerInfoContext {
	params: ImmutableGetPlayerInfoParams,
	results: MutableGetPlayerInfoResults,
	state: ImmutablezentangleState,
}

fn view_get_player_info_thunk(ctx: &ScViewContext) {
	ctx.log("zentangle.viewGetPlayerInfo");
	let f = GetPlayerInfoContext {
		params: ImmutableGetPlayerInfoParams { proxy: params_proxy() },
		results: MutableGetPlayerInfoResults { proxy: results_proxy() },
		state: ImmutablezentangleState { proxy: state_proxy() },
	};
	ctx.require(f.params.player_address().exists(), "missing mandatory playerAddress");
	view_get_player_info(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.viewGetPlayerInfo ok");
}

pub struct GetPlaysPerImageContext {
	params: ImmutableGetPlaysPerImageParams,
	results: MutableGetPlaysPerImageResults,
	state: ImmutablezentangleState,
}

fn view_get_plays_per_image_thunk(ctx: &ScViewContext) {
	ctx.log("zentangle.viewGetPlaysPerImage");
	let f = GetPlaysPerImageContext {
		params: ImmutableGetPlaysPerImageParams { proxy: params_proxy() },
		results: MutableGetPlaysPerImageResults { proxy: results_proxy() },
		state: ImmutablezentangleState { proxy: state_proxy() },
	};
	ctx.require(f.params.image_id().exists(), "missing mandatory imageId");
	view_get_plays_per_image(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.viewGetPlaysPerImage ok");
}

pub struct GetResultsContext {
	params: ImmutableGetResultsParams,
	results: MutableGetResultsResults,
	state: ImmutablezentangleState,
}

fn view_get_results_thunk(ctx: &ScViewContext) {
	ctx.log("zentangle.viewGetResults");
	let f = GetResultsContext {
		params: ImmutableGetResultsParams { proxy: params_proxy() },
		results: MutableGetResultsResults { proxy: results_proxy() },
		state: ImmutablezentangleState { proxy: state_proxy() },
	};
	ctx.require(f.params.image_id().exists(), "missing mandatory imageId");
	view_get_results(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("zentangle.viewGetResults ok");
}