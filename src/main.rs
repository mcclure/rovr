use mlua::{Lua};
use mlua::prelude::LuaError;

fn main() -> Result<(), LuaError> {
	let lua = Lua::new();

	lua.load(
        r#"
            print("Printing from Lua")
        "#,
    )
    .set_name("example code")?
    .exec()?;

    Ok(())
}
