use std::collections::HashMap;

fn main() {
    let mut v: Vec<i8> = vec![63, -2, 22, -11, 48, 49, 100, 100];
    v.sort_by(|a, b| a.cmp(b));

    let median = v[v.len() / 2];

    let mut map = HashMap::new();
    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut mode: i8 = 0;
    let mut i = 0;
    for (key, value) in map {
        if value > i {
            mode = key;
            i = value;
        }
    }

    println!("The median value is {median}, the mode value is {mode}");
}
