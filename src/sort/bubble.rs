use std::time::Instant;
pub fn sort(input: Vec<u16>) -> String {
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = input.to_vec();
    let mut swapped: bool;
    // Loop over every single element
    for _ in output.to_vec() {
        swapped = false;
        // Go through every number and swap them if they need to be swapped
        for j in 0..(output.len() - 1) {
            
            if output[j] > output[j + 1] {
                output.swap(j, j + 1);
                swapped = true;
            }
            // sleep_ms(16);
            // create_bars(&output, vec![output[j], output[j + 1]]);
            comparisons += 1;
        }
        // Nothing was swapped so therefore the array is sorted
        if swapped == false {
            break;
        }
    }
    format!(
        "\nInput: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
    )
}