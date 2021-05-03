use std::time::Instant;
use std::thread::sleep_ms;
use crate::create_bars;
pub fn sort(input: Vec<u16>) -> String {
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = input.to_vec();
    let len = output.len();
    // Loop over every single element
    for left in 0..len {
        // make the smallest known element the current one
        let mut smallest = left;
        // Find if there is a smaller element in the list
        for right in left..len {
            if output[right] < output[smallest] {
                smallest = right;
            }
            // create_bars(output.to_vec());
            comparisons += 1;
        }
        // Swap it with the slot
        output.swap(smallest, left);
    }
    format!(
        "Input: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
    )
}
