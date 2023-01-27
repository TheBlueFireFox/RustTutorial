use std::vec;

pub fn run() {
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    slices();
}

pub fn simple_borrow() {
    // Immutable (n times)
    let var = 0;
    let var_borrow = &var;
    // *var_borrow += 1;

    // Mutable (1 time)
    let mut var = 0;
    let var_mut_borrow = &mut var;
    let var_borrow = &var;

    // *var_mut_borrow += 1;
}

pub fn lifetimes() {
    {
        let var = 0; // 'var
        {
            let var_borrow = & /* 'var */ var; // 'var_borrow

            /*
             * stuff happends
             */

            /*
             * more stuff happends
             */
        } // 'var_borrow ends
    } // 'var ends
}

pub fn strings() {
    //
    // Owned value (Copy to heap)
    let var: String = "Hier könnte Ihre Werbung stehen.".to_owned();

    let var_borrow = &var;

    let var_borrow_str: &str = &var;
    let var_borrow_str = &var[..]; // slice
}

// pub fn illegal_return() -> &str {
//     let var = "hi".to_owned(); // 'var
//     let var_borrow = &var;
//
//     var_borrow
//     // 'var ends here
// }

pub fn special_legal_return() -> &'static str {
    const VAR: &'static str = "hi"; // 'var == 'static
                                    // ==> infinite lifetime because string is part
                                    // of program itself
    let var_borrow = &VAR;

    var_borrow
    // 'var ends here
}

pub fn slices() {
    let var = "Hier könnte Ihre Werbung stehen.".to_owned();
    println!("{var}");

    let var = &var[..];
    println!("{var}");

    // let hier = &var[..4];
    // println!("{hier}");
    // let hier = &var[..=3];
    // println!("{hier}");

    // let könnte = &var[5..12];
    // println!("{könnte}");
    // let könnte = &var[5..][..7];
    // println!("{könnte}");

    // NOT THE SAME => char == u32
    fn inner_char_slice(var: &mut [char]) {
        //
    }

    // str uses utf8
    fn inner_str(var: &mut str) {
        //
    }
}

pub fn mutable_slices() {
    fn inner_slice(var: &mut [usize]) {
        //
    }

    let mut var = [0; 5];

    inner_slice(&mut var);
}
