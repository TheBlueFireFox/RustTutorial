pub fn run() {
    // controll flow:
    if 0 < 4 {
        // something
    } else if 0 == 4 {
        // no comment
    } else {
        // must have been a bit flip
    }

    let var = if 0 < 4 { true } else { false };

    // match
    let var = 4;

    let _ = match var {
        1 => "1",
        2 => "2",
        3 | 4 => "3 or 4",
        5..=8 => "5,6,7,8",
        _ => "everything else", // must be exhaustive
    };

    let var = "hi";

    let _ = match var {
        "hi" => true,
        "or" => false,
        _ => false,
    };

    // Loops:
    for i in 0..100 {}

    let var = [1, 2, 3, 4, 5, 6];

    for elem in var {
        continue;
    }

    while 0 < 4 {
        break;
    }

    loop {
        break;
    }

    let var = 'loop_label: loop {
        loop {
            break 'loop_label 0;
        }
    };
}
