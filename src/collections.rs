pub fn vector() {
    // let mut v = Vec::new();
    let mut v = Vec::with_capacity(3);
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![5, 6, 7];

    let v = vec![0; 3]; // [0,0,0]
}

pub fn maps_and_sets() {
    // import stuff from crates etc.
    use std::collections::{HashMap, HashSet};

    let map: HashMap<u8, String> = HashMap::new();
    let set: HashSet<u8> = HashSet::new();
}
