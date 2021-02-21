use mlua::Lua;
use mlua::prelude::{LuaTable, LuaError};

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	let table = lua.create_table()?;

	Ok(table)
}
