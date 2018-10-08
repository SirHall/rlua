extern crate rlua;

use std::sync::mpsc::{channel, TryRecvError};
use std::ops::Deref;
use std::time::{Instant, Duration};
use rlua::{Lua, Debug, HookOptions, Error, Value};

#[test]
fn line_counts() {
    let code = r#"local x = 2 + 3
    local y = x * 63
    local z = string.len(x..", "..y)
    "#;

    let (sx, rx) = channel();
    let lua = Lua::new();
    lua.set_mut_hook(HookOptions {
        lines: true, ..Default::default()
    }, move |debug: &Debug| {
        let _ = sx.send(debug.curr_line);
        Ok(())
    });
    let _: () = lua.exec(code, None).expect("exec error");

    assert_eq!(rx.try_recv(), Ok(1));
    assert_eq!(rx.try_recv(), Ok(2));
    assert_eq!(rx.try_recv(), Ok(3));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn function_calls() {
    let code = r#"local v = string.len("Hello World")"#;

    let (sx, rx) = channel();
    let lua = Lua::new();
    lua.set_mut_hook(HookOptions {
        calls: true, ..Default::default()
    }, move |debug: &Debug| {
        let _ = sx.send(debug.to_owned());
        Ok(())
    });
    let _: () = lua.exec(code, None).expect("exec error");

    assert_eq!(rx.recv().unwrap().what.as_ref().unwrap().as_ref(), "main");
    assert_eq!(rx.recv().unwrap().name.as_ref().unwrap().as_ref(), "len");
}

#[test]
fn error_within_hook() {
    let lua = Lua::new();
    lua.set_hook(HookOptions {
        lines: true, ..Default::default()
    }, |_debug: &Debug| {
        Err(Error::RuntimeError("Something happened in there!".to_string()))
    });

    let err = lua.exec::<_, ()>("x = 1", None).expect_err("panic didn't propagate");
    match err {
        Error::CallbackError { cause, .. } => match cause.deref() {
            Error::RuntimeError(s) => assert_eq!(s, "Something happened in there!"),
            _ => panic!("wrong callback error kind caught")
        },
        _ => panic!("wrong error kind caught")
    }
}

#[test]
fn limit_execution_time() {
    let code = r#"
    while true do
        x = x + 1
    end
    "#;
    let start = Instant::now();

    let lua = Lua::new();
    let _ = lua.globals().set("x", Value::Integer(0));
    lua.set_hook(HookOptions {
        after_counts: true, count: 30, ..Default::default()
    }, move |_debug: &Debug| {
        if start.elapsed() >= Duration::from_millis(500) {
            Err(Error::RuntimeError("time's up".to_string()))
        } else {
            Ok(())
        }
    });

    let _ = lua.exec::<_, ()>(code, None).expect_err("timeout didn't occur");
    assert!(start.elapsed() < Duration::from_millis(750));
    //println!("{}", lua.globals().get::<_, i64>("x").unwrap());
}

#[test]
fn hook_removal() {
    let code = r#"local x = 1"#;
    let lua = Lua::new();

    lua.set_hook(HookOptions {
        after_counts: true, count: 1, ..Default::default()
    }, |_debug: &Debug| {
        Err(Error::RuntimeError("this hook should've been removed by this time".to_string()))
    });
    assert!(lua.exec::<_, ()>(code, None).is_err());

    lua.remove_hook();
    assert!(lua.exec::<_, ()>(code, None).is_ok());
}
