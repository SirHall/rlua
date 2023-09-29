use rlua::{Lua, Result};

fn main() -> Result<()> {
    // if !cfg!(not(feature = "builtin-lua53-eris")) {
    //     panic!(
    //         "Must enable the eris lua engine feature to run this example: '--features rlua-lua53-eris'"
    //     );
    // }

    // You can create a new Lua state with `Lua::new()`.  This loads the default Lua std library
    // *without* the debug library.  You can get more control over this with the other
    // `Lua::xxx_new_xxx` functions.
    let lua = Lua::new();

    // In order to interact with Lua values at all, you must do so inside a callback given to the
    // `Lua::context` method.  This provides some extra safety and allows the rlua API to avoid some
    // extra runtime checks.

    let persisted_binary = lua.context(|lua_ctx| {
        // You can get and set global variables.  Notice that the globals table here is a permanent
        // reference to _G, and it is mutated behind the scenes as Lua code is loaded.  This API is
        // based heavily around sharing and internal mutation (just like Lua itself).

        let globals = lua_ctx.globals();

        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;

        let perms = lua_ctx.create_table()?;

        let persisted = lua_ctx.persist(perms, rlua::Value::Table(globals))?;

        let bytes = persisted.as_bytes();

        println!("Persisted: {:?}", &bytes);

        Ok(bytes.to_vec())
    })?;

    lua.context(|lua_ctx| {
        let perms = lua_ctx.create_table()?;
        let globals = lua_ctx.globals();

        let unpersisted = lua_ctx.unpersist(perms, persisted_binary_str)?;

        Ok(())
    })?;

    Ok(())
}
