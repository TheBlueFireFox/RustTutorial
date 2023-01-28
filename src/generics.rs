// like interface
trait Sound {
    fn sounds(&self);
}

#[derive(Debug)]
struct Human {
    age: u8,
    name: String,
}

impl Sound for Human {
    fn sounds(&self) {
        // do something
    }
}

#[derive(Debug)]
struct Cat {
    age: u8,
    name: String,
}

fn uses_traits_impl<T>(thing: &impl Sound)
where
    T: Sound,
{
    thing.sounds();
}

fn uses_impl_return() -> impl Sound {
    Human {
        age: 0,
        name: "Peter".to_string(),
    }
}

fn uses_traits_gen<T>(thing: &T)
where
    T: Sound,
{
    thing.sounds();
}

fn uses_traits_dyn(thing: &dyn Sound) {
    thing.sounds();
}

fn uses_traits_two<T>(thing: &T)
where
    T: Sound + std::fmt::Debug,
{
    thing.sounds();
    println!("{:?}", thing);
}

fn lifetimes_explicit<'a>(s: &'a str) -> &'a str {
    // "  asdf  " => "asdf"
    s.trim()
}

fn lifetimes_no(s: &str) -> &'_ str {
    // "  asdf  " => "asdf"
    s.trim()
}
