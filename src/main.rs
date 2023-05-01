use std::collections::{BTreeMap};
use itertools::iproduct;
use counter::Counter;

fn main() {

    let mut ons = BTreeMap::<usize, Vec<usize>>::new();
    let mut offs = BTreeMap::<usize, Vec<usize>>::new();

    for on in 10..41 {
        for off in on + 1 .. 41 {
            let dc = on as f64 / (on as f64 + off as f64) * 100.0;
            if dc as f32 % 1.0 == 0.0 {
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
            if on.0 == &11usize {
                println!("{:?}", all_offs);
            }
            for (a, b, c, d) in iproduct!(&all_offs[0], &all_offs[1], &all_offs[2], &all_offs[3]) {
                    let on_count = [a, b, c, d].into_iter().collect::<Counter<&usize>>();
                    if on_count.len() == 2 {
                        println!("{:?} {:?}", on.0, on_count);

                    }
            }
        }
    }

    for on in &offs {
        let check_contiguous = (1..4).map(|z| offs.contains_key(&(on.0 + z))).collect::<Vec<bool>>();
        if !check_contiguous.contains(&false) {
            let all_offs = (0usize..4).map(|z| offs[&(on.0 + z)].to_owned()).collect::<Vec<Vec<usize>>>();
            for (a, b, c, d) in iproduct!(&all_offs[0], &all_offs[1], &all_offs[2], &all_offs[3]) {
                    let on_count = [a, b, c, d].into_iter().collect::<Counter<&usize>>();
                    if on_count.len() == 2 {
                        println!("{:?} {:?}", on.0, on_count);

                    }
            }
        }
    }


}