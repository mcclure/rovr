#![allow(unused_parens)]

mod api;

use mlua::prelude::LuaTable;
use std::env;
use mlua::{Lua};
use mlua::prelude::LuaError;

fn main() -> Result<(), LuaError> {
    let mut args = env::args().fuse();
    let arg0 = args.next();
    let arg1 = args.next();

    if (match arg1 {
        None => false, Some(ref x) => x == "-v" || x == "--version"
    }) {
        println!("ROVR 0.0.1");
        return Ok(()); // Print version and abort
    }

    // FIXME handle --console

    loop { // Run actual program
        let lua = Lua::new();

        // TODO: luax_setmainthread(L);

        let arg_table = lua.create_table()?;
        
        match arg0 { None => (), Some(x) => arg_table.set("exe", x)? }
        match arg1 { None => (), Some(x) => arg_table.set(0, x)? }

        // FIXME set "cookie"
        {
            let mut idx = 1;
            for arg in args {
                arg_table.set(idx, arg)?;
                idx += 1;
            }
        }

        let globals = lua.globals();
        globals.set("arg", arg_table)?;

        let package:LuaTable = globals.get("package")?;
        let preload:LuaTable = package.get("preload")?;
        api::load(&lua, preload)?;

        // TODO lovrModules

        let boot_lua = include_str!("resources/boot.lua");

        lua.load(
            boot_lua
        )
        .set_name("@boot.lua")?
        .exec()?;

        break;
    }

    Ok(())
}
