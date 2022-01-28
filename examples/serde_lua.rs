use std::f32;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

use rlua::{prelude::*, StdLib};

#[macro_export]
macro_rules! serde_lua_module {
    ($module_name:literal $($name:literal => $func:ident, )* ) => {
        struct $module_name {}

        impl $module_name
        {
            pub fn register()
        }
    };
}

fn test(lua : &mut Lua) -> () { () }

fn main() -> Result<(), String>
{
    let mut lua = Lua::new_with(StdLib::ALL_NO_DEBUG);

    Ok(())
}
