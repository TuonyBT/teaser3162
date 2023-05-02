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

    let on_group = find_pairs(&ons);
    if on_group.len() == 1 {
        let duty_times = on_group[0].iter()
        .map(|z| z.0 * 100 / (z.0 + z.1))
        .collect::<Vec<usize>>();
        println!("First set duty times: {:?}", duty_times);
    } 

    let off_group = find_pairs(&offs);
    if off_group.len() == 1 {
        let duty_times = off_group[0].iter()
        .map(|z| z.1 * 100 / (z.0 + z.1))
        .collect::<Vec<usize>>();
        println!("Second set duty times: {:?}", duty_times);
    } 


}

fn find_pairs(state_times: &BTreeMap::<usize, Vec<usize>>) -> Vec<Vec<(usize, usize)>> {
    let mut stps = Vec::<Vec<(usize, usize)>>::new();
    for st in state_times {
        let check_contiguous = (1..4).map(|z| state_times.contains_key(&(st.0 + z))).collect::<Vec<bool>>();
        if !check_contiguous.contains(&false) {
            let all_comps = (0usize..4).map(|z| state_times[&(st.0 + z)].to_owned()).collect::<Vec<Vec<usize>>>();
            for (a, b, c, d) in iproduct!(&all_comps[0], &all_comps[1], &all_comps[2], &all_comps[3]) {
                let st_count = [a, b, c, d].into_iter().collect::<Counter<&usize>>();
                if st_count.values().collect::<Vec<&usize>>() == vec![&2, &2] {
                    let state_pairs = (0usize..4).map(|z| (st.0 + z)).collect::<Vec<usize>>();
                    stps.push(state_pairs.iter().zip([a, b, c, d])
                    .map(|(&x, &y)| (x, y))
                    .collect::<Vec<(usize, usize)>>());
                }
            }
        }
    }
    stps
}