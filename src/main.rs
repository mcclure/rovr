#![allow(unused_parens)]

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

        // TODO lovrModules

        lua.load(
            r#"
                print("Printing from Lua", arg[1])
            "#,
        )
        .set_name("example code")?
        .exec()?;

        break;
    }

    Ok(())
}
