use mlua::prelude::*;

pub fn handle() -> LuaResult<()> {
    let lua = Lua::new();

    lua.load("print('Hello Lua')").exec()?;

    Ok(())
}
