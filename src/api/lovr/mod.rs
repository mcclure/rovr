// This file contains Lua functions

pub mod filesystem;

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaResult, LuaError, LuaValue, LuaMultiValue};
use crate::api::int_or_nil;

fn _setConf(lua: &Lua, conf: LuaValue) -> LuaResult<()> {
	lua.set_named_registry_value("_lovrconf", conf)?;
	Ok(())
}

fn getVersion(lua: &Lua, _: ()) -> LuaResult<LuaMultiValue> {
	use crate::core::{LOVR_VERSION, ROVR_VERSION};

	let options: Vec<LuaValue> = vec![LuaValue::Integer(LOVR_VERSION.0),
									  LuaValue::Integer(LOVR_VERSION.1),
									  LuaValue::Integer(LOVR_VERSION.2),
									  LuaValue::Nil,
									  LuaValue::String(lua.create_string("rovr")?),
									  int_or_nil(ROVR_VERSION.0),
									  int_or_nil(ROVR_VERSION.1),
									  int_or_nil(ROVR_VERSION.2),
									  ];
	Ok(LuaMultiValue::from_vec(options))
}

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	let table = lua.create_table()?;

	table.set("_setConf", lua.create_function(_setConf)?)?;
	table.set("getVersion", lua.create_function(getVersion)?)?;

	Ok(table)
}
