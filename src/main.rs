use std::collections::{BTreeMap};
use itertools::iproduct;

fn main() {

    let mut ons = BTreeMap::<usize, Vec<usize>>::new();
    let mut offs = BTreeMap::<usize, Vec<usize>>::new();

    for on in 10..40 {
        for off in on + 1 .. 41 {
            let dc = on as f64 / (on as f64 + off as f64) * 100.0;
            if dc % 1.0 == 0.0 {
                ons.entry(on).or_insert(Vec::new()).push(off);
                offs.entry(off).or_insert(Vec::new()).push(on);
            }

        }
    }
    //println!("{:?}", ons.keys());
    //println!("{:?}", offs);
    for on in &ons {
        let check_contiguous = (1..4).map(|z| ons.contains_key(&(on.0 + z))).collect::<Vec<bool>>();
        if !check_contiguous.contains(&false) {
            let all_offs = (0usize..4).map(|z| ons[&(on.0 + z)].to_owned()).collect::<Vec<Vec<usize>>>();
            for a in &all_offs[0] {
                for b in &all_offs[0] {
                    for c in &all_offs[0] {
                        for d in &all_offs[0] {
                            println!("{:?} {:?}", on.0, vec![a, b, c, d]);

                        }
                    }
                }
            }
        }
    }
}