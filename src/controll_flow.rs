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
