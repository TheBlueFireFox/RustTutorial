pub fn run() {
    // enums
    let four = IpAddrKind::V4;
    println!("{:?}", four);

    let four = IpAddrKindWithStruct::V4 {
        ip: "127.0.0.1".to_string(),
        port: 8080,
    };

    println!("{:?}", four);

    let six = IpAddrKindWithStruct::V6(
        0x2001, 0x0db8, 0x3c4d, 0x0015, 0x0000, 0x0000, 0x1a2f, 0x1a2b,
    );

    println!("{:?}", six);

    // match
    match_it(four);
    match_it(six);

    // if let
    let_if_it(IpAddrKind::V4);

    error_handling();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn new() -> Self {
        IpAddrKind::V4
    }
}

#[derive(Debug, Clone)]
enum IpAddrKindWithStruct {
    V4 { ip: String, port: u16 },
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}

fn match_it(ip: IpAddrKindWithStruct) {
    // match
    match ip {
        IpAddrKindWithStruct::V4 { ip, port } => println!("{ip}:{port}"),
        IpAddrKindWithStruct::V6(a, b, c, d, e, f, g, h) => {
            println!("{a:X}:{b:X}:{c:X}:{d:X}:{e:X}:{f:X}:{g:X}:{h:X}")
        }
    }
}

fn let_if_it(ip: IpAddrKind) {
    if let IpAddrKind::V4 = ip {
        println!("ip v4");
    }
}

// #[allow(unconditional_panic)]
fn error_handling() {
    // Error Handling
    //
    // unrecoverable:
    // panic!();
    //
    // Ex. Illegal access of memory
    //
    // let arr = [0; 4];
    // let _ = arr[99];
    //
    // error: this operation will panic at runtime
    //   --> src/enums.rs:71:13
    //    |
    // 71 |     let _ = arr[99];
    //    |             ^^^^^^^ index out of bounds: the length is 4 but the index is 99
    //    |
    //    = note: `#[deny(unconditional_panic)]` on by default
    //
    // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 99', src/enums.rs:73:13
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:147:5
    //    3: common::enums::error_handling
    //              at ./src/enums.rs:73:13
    //    4: common::enums::run
    //              at ./src/enums.rs:26:5
    //    5: common::main
    //              at ./src/main.rs:22:5
    //    6: core::ops::function::FnOnce::call_once
    //              at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    // recoverable -- use this in read code
    //
    // #[must_use]
    // enum Result<E, T> {
    //     Ok(E),
    //     Err(T),
    // }

    fn result_gen() -> Result<u8, &'static str> {
        let mut res = Ok(0);
        // ...
        // ...
        // ...

        res = Err("There was an error with your furry here");

        res
    }

    fn result_return() -> Result<(), &'static str> {
        let res = result_gen();

        // via match
        let r = match res {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        // via question mark operator
        let r = result_gen()?;

        if let Ok(r) = result_gen() {
            // via if let pattern matching
        }

        Ok(())
    }
}

fn options() {
    // Optional types => null vs no null

    // works because
    // 1) it it marked must_use
    // 2) it is a different type then underlying one

    // #[must_use]
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    fn option_find(s: &[u32], idx: usize) -> Option<u32> {
        if s.len() < idx {
            return None;
        }
        Some(s[idx])
    }

    fn option_return() -> Option<()> {
        let x = [1, 2, 3, 4];

        let f = option_find(&x, 2);

        let y: u32 = 33;

        // let b = y + f;
        //
        // needs to be unwrapped
        // let b = y + f?;
        //
        // if let Some(v) = f {
        //     let b = y + v;
        // }
        //

        // let b = y + f.unwrap(); // will panic if f is None
        // let b = y + f.expect("Some message"); // will panic if f is None

        Some(())
    }
}
