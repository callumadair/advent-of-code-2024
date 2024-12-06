use std::{
    fs::File,
    io::Read,
    path::Path,
    str::FromStr,
};

use color_eyre::Result;
use regex::{
    Match,
    Regex,
};

fn main()
{
    let file_contents =
        read_file(Path::new("./src/input.txt")).expect("Failed reading input file.");
    let part_two_total =
        calculate_product_total(&file_contents).expect("Failed calculating total.");
    println!("Part One Total is: {part_two_total}");

    let part_two_total = calculate_product_total_with_conditionals(&file_contents)
        .expect("Failed calculating total with conditionals");
    println!("Part Two Total is: {part_two_total}");
}
fn read_file(path: &Path) -> Result<String>
{
    let mut input_file = File::open(path)?;
    let mut output_string = String::new();
    let _file_contents = input_file.read_to_string(&mut output_string)?;

    Ok(output_string)
}

fn calculate_product_total(input: &str) -> Result<usize>
{
    let re = Regex::new(r"mul\(\d+,\d+\)")?;
    let matches = re.find_iter(input).collect::<Vec<Match>>();
    parse_inner_regex(matches)
}

fn parse_inner_regex(matched_outer_regex: Vec<Match>) -> Result<usize>
{
    let inner_re = Regex::new(r"\d+")?;
    let total = matched_outer_regex
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

fn calculate_product_total_with_conditionals(input: &str) -> Result<usize>
{
    let conditional_regex =
        Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)|(?<yes>do\(\))|(?<no>don't\(\))")?;
    let total = {
        let mut activated = true;
        let mut total = 0;
        for capture in conditional_regex.captures_iter(input)
        {
            if capture.name("yes").is_some()
            {
                activated = true;
                continue;
            }
            if capture.name("no").is_some()
            {
                activated = false;
                continue;
            }

            if activated
            {
                total += capture["left"].parse::<usize>()? * capture["right"].parse::<usize>()?;
            }
        }
        total
    };

    Ok(total)
}
