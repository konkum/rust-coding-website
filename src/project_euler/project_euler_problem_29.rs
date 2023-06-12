use std::collections::HashSet;

pub fn distinct_powers() {
    let mut powers = HashSet::new();

    for a in 2..101 {
        for b in 2..101 {
            let term = (a as f64).powf((b as f64));
            let bits: u64 = unsafe { std::mem::transmute(term) };
            powers.insert(bits);
        }
    }

    println!("{}", powers.len());
}