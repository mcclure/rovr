use mlua::Lua;
use mlua::prelude::{LuaError, LuaResult};
use crate::api::LuaTable;

fn func1(_: &Lua, _: ()) -> LuaResult<()> {
	println!("Test");

    Ok(())
}

fn func2(_: &Lua, _: ()) -> LuaResult<()> {
    Ok(())
}

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	let table = lua.create_table()?;
	table.set("func1", lua.create_function(func1)?)?;
	table.set("func2", lua.create_function(func2)?)?;

	Ok(table)
}
