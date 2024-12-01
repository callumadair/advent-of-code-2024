use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
    str::FromStr,
};

use color_eyre::Result;

const INPUT_LINE_DELIMITER: &str = "   ";

fn main()
{
    let mut vecs =
        read_file(Path::new("./src/input.txt")).expect("Failed reading input file for distances.");
    let distance = calculate_distance(&mut vecs.0, &mut vecs.1);
    let similarity = calculate_similarity(vecs.0, vecs.1);
    println!("Distance between the two is: {distance}");
    println!("Similarity score between the two is: {similarity}")
}
fn read_file(path: &Path) -> Result<(Vec<usize>, Vec<usize>)>
{
    let input_file = File::open(path)?;
    let (mut left, mut right) = (Vec::new(), Vec::new());
    let buf_reader = BufReader::new(input_file);

    for line in buf_reader.lines()
    {
        let unwrapped_line = line?;
        let line_vals = unwrapped_line
            .split(INPUT_LINE_DELIMITER)
            .collect::<Vec<&str>>();

        left.push(usize::from_str(line_vals[0])?);
        right.push(usize::from_str(line_vals[1])?);
    }

    Ok((left, right))
}
fn calculate_distance(
    left: &mut [usize],
    right: &mut [usize],
) -> usize
{
    left.sort();
    right.sort();
    assert_eq!(left.len(), right.len());
    // Find difference and return the sum
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn calculate_similarity(
    left: Vec<usize>,
    right: Vec<usize>,
) -> usize
{
    left.into_iter()
        .map(|l| {
            let repeat_count = right
                .clone()
                .into_iter()
                .filter(|r| l == *r)
                .collect::<Vec<usize>>()
                .len();
            l * repeat_count
        })
        .sum()
}
