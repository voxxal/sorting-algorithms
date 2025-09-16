mod display;
mod sort;
use display::bar::create_bars;
use rand::{seq::SliceRandom, thread_rng};
use sort::*;
use std::collections::HashMap;

//TODO Comment code so you can understand it tommorow :)

fn main() {
    let mut sorting_map = HashMap::new();
    sorting_map.insert("selection", selection::sort);
    let _sorting: [fn(Vec<u16>) -> String; 6] = [
        selection::sort,
        stalin::sort,
        insertion::sort,
        bogo::sort,
        bubble::sort,
        quantum_bogo::sort,
    ];
    let mut a = Vec::new();
    for i in 1..150 {
        if i % 2 == 0 {
            a.push(i);
        }
    }
    // let mut a = (1..50).collect::<Vec<u16>>();
    a.shuffle(&mut thread_rng());
    println!("{}", insertion::sort(a))
}
