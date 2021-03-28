// This file contains Lua functions

pub mod display;

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaResult, LuaError, LuaValue, LuaMultiValue};
use crate::api::int_or_nil;

fn getVersion(lua: &Lua, _: ()) -> LuaResult<LuaMultiValue> {
	use crate::core::{ROVR_VERSION};

	let options: Vec<LuaValue> = vec![int_or_nil(ROVR_VERSION.0),
									  int_or_nil(ROVR_VERSION.1),
									  int_or_nil(ROVR_VERSION.2),
									  ];
	Ok(LuaMultiValue::from_vec(options))
}

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	let table = lua.create_table()?;

	table.set("getVersion", lua.create_function(getVersion)?)?;

	Ok(table)
}
