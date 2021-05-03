use crate::create_bars;
use std::thread::sleep_ms;
use std::time::Instant;
pub fn sort(input: Vec<u16>) -> String {
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = input.to_vec();
    for key in 1..output.len() {
        let mut j = key;
        while j > 0 && output[j] < output[j - 1] {
            output.swap(j, j - 1);
            // create_bars(&output, vec![output[j], output[j - 1],output[key]]);
            comparisons += 1;
            j = j - 1;
        }
    }

    format!(
        "Input: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
    )
}
