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

fn option_and_result() {
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

        Ok(())
    }

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
}
