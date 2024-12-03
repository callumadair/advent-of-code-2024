use std::{
    fs::File,
    io::{
        BufReader,
        Read,
    },
    path::Path,
    str::FromStr,
};

use color_eyre::Result;
use regex::Regex;

fn main()
{
    let file_contents =
        read_file(Path::new("./src/input.txt")).expect("Failed reading input file.");
    let total = calculate_product_total(file_contents).expect("Failed calculating total.");
    println!("Total is: {total}");
}
fn read_file(path: &Path) -> Result<String>
{
    let mut input_file = File::open(path)?;
    let mut output_string = String::new();
    let _file_contents = input_file.read_to_string(&mut output_string)?;

    Ok(output_string)
}

fn calculate_product_total(input: String) -> Result<usize>
{
    let re = Regex::new(r"mul\(\d+,\d+\)")?;
    let matches = re.find_iter(&input).collect::<Vec<_>>();
    dbg!(&matches);
    let inner_re = Regex::new(r"\d+")?;
    let total = matches
        .iter()
        .map(|val| {
            let inner_matches = inner_re
                .find_iter(val.as_str())
                .map(|res| res.as_str())
                .collect::<Vec<&str>>();
            let nums = inner_matches
                .iter()
                .map(|val| usize::from_str(val).unwrap())
                .collect::<Vec<usize>>();
            nums.iter().product::<usize>()
        })
        .sum();

    Ok(total)
}
