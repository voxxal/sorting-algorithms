use std::time::Instant;
pub fn sort(input: Vec<u32>) -> String {
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = input.to_vec();
    for i in output {

    }
    format!(
        "Input: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
    )
}
