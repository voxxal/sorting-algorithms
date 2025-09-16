use std::convert::TryFrom;
use std::time::Instant;
// pub struct Result {
//     pub input: Vec<u32>,
//     pub output: Vec<u32>,
//     pub elapsed: f32,
//     pub comparisons: u32,
//     pub removed: u32,
// }
pub fn sort(input: Vec<u16>) -> String {
    //TODO REMOVE THE OBJECTS, DONT ADD THE ONES IN LINE
    let mut comparisons = 0;
    let elap = Instant::now();
    let mut output = vec![];
    for entry in input.to_vec() {
        if output.is_empty() || &entry >= output.last().unwrap() {
            output.push(entry);
        }
        comparisons += 1;
    }
    let removed = u32::try_from(input.len() - output.len()).unwrap_or(0);
    format!(
        "Input: {:?}\nOutput: {:?}\nElapsed: {}\nComparisons: {}\nRemoved: {}\n",
        input,
        output,
        elap.elapsed().as_secs_f32(),
        comparisons,
        removed
    )
}
