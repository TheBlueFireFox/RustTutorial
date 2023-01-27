pub fn types() {
    let var = 0;
    let var: i32 = 0;
    let var = 0i32;

    // Integer fix width:
    // unsigned: u8, u16, u32, u64, u128
    // signed  : i8, i16, i32, i64, i128
    //
    // Integer arch width:
    // unsigned: usize
    // signed  : isize

    // Float:
    let var = 0.0;
    let var: f32 = 0.0;
    let var = 0.0f32;

    // Boolean
    let var = false;
    let var: bool = true;

    // Character:
    let var = 'z';
    let var: char = '🍆';

    // Tuple:
    let var = (0, 0);
    let (var, _): (i32, i32) = (0, 0);
    let var = (0, 0);
    let _ = var.0;
    let _ = var.1;

    // Empty Tuple:
    let var = ();

    // Array:
    let var = [0, 0, 0, 0];
    let var = [0; 4];
    let var: [i32; 4] = [0, 0, 0, 0];
    let var: [_; 4] = [0, 0, 0, 0];
    // let var: [i32; _] = [0, 0, 0, 0];

    // Strings immutable (immutable slice):
    let var = "🍆💦🍑";
    let var: &'static str = "Lorem Ipsum";
    // Strings mutable
    let var = "Lorem Ipsum".to_string();
    let var = String::from("Lorem Ipsum");
    // let var: String = "Lorem Ipsum";

    // struct
    struct Struct {
        var: usize,
    }
}

pub fn functions() {
    // pub / pub (crate) ...
    fn inner_ret(var: i32) -> i32 {
        var
    }

    fn inner_ret_too(var: i32) -> i32 {
        return var;
    }

    // we filled in
    let inner = |var: i32| -> i32 { var };

    // infered
    let inner = |var| var;
    inner(0);

    // infered too
    let inner = |var: _| var;
    inner(0);

    fn inner_param(var: String) {
        // String was moves into function
        // so it lives in here now
    }

    fn inner_param_ref(var: &String) {
        // String is an immutable reference here
    }

    fn inner_param_ref_mut(var: &mut String) {
        // String is a mutable reference here
    }

    let mut var = "Something else".to_string();

    inner_param_ref(&var);
    inner_param_ref(&mut var);
    inner_param(var);
    // illegal bevause var was moved
    // inner_param_ref(&var);
}
