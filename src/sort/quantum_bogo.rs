use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
pub fn sort(input: Vec<u16>) -> String {
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = input.to_vec();
    let len = output.len();
    loop {
        // Randomize the array
        output.shuffle(&mut thread_rng());
        comparisons += 1;
        // If the array has 1 or 0 elements, its already sorted
        if len == 1 || len == 0 {
            break;
        }
        let mut cont : bool = false;
        // Check if one element is greater than the other
        for i in 1..len {
            if output[i - 1] > output[i] {
                cont = true;
            }
        };
        // Destroy the universe
        if cont {
            panic!("Failed to destroy universe.");
        }
        // The array is sorted!
        break;
    }
    format!(
        "Input: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
    )
}
