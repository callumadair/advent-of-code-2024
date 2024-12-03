use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Lines,
    },
    path::Path,
    str::FromStr,
    usize,
};

use color_eyre::Result;

const INPUT_LINE_DELIMITER: &str = " ";

fn main()
{
    let safety_value =
        calculate_safety(Path::new("./src/input.txt")).expect("Failed calculating safety total.");
    println!("The safety value for this input is: {safety_value}");
    let safety_value = calculate_safety_with_dampener(Path::new("./src/input.txt"))
        .expect("Failed calculating safety total with safety dampener.");
    println!("The safety value for this input with the safety dampener is: {safety_value}");
}

fn get_lines(path: &Path) -> Result<Lines<BufReader<File>>>
{
    let input_file = File::open(path)?;
    let buf_reader = BufReader::new(input_file);
    Ok(buf_reader.lines())
}

fn calculate_safety(path: &Path) -> Result<usize>
{
    let mut safety_count = 0_usize;

    'line_reader: for line in get_lines(path)?
    {
        let line_vals = collect_line_vals(line?)?;

        for i in 1..line_vals.len() - 1
        {
            let last = line_vals[i - 1];
            let cur = line_vals[i];
            let next = line_vals[i + 1];

            // Carry out all the checks
            if (last < cur && cur > next)
                || (last > cur && cur < next)
                || (last.abs_diff(cur) < 1 || last.abs_diff(cur) > 3)
                || (cur.abs_diff(next) < 1 || cur.abs_diff(next) > 3)
            {
                continue 'line_reader;
            }
        }
        safety_count += 1;
    }

    Ok(safety_count)
}

fn collect_line_vals(line: String) -> Result<Vec<usize>>
{
    // Parse the line into a vec of usize vals.
    let line_vals = line
        .split(INPUT_LINE_DELIMITER)
        .collect::<Vec<&str>>()
        .iter()
        .map(|val| usize::from_str(val).unwrap())
        .collect::<Vec<usize>>();
    Ok(line_vals)
}

enum ValueDirection
{
    None,
    Ascending,
    Descending,
}

impl ValueDirection
{
    fn is_assigned(&self) -> bool
    {
        match *self
        {
            ValueDirection::None => false,
            ValueDirection::Ascending => true,
            ValueDirection::Descending => true,
        }
    }
}

fn calculate_safety_with_dampener(path: &Path) -> Result<usize>
{
    let mut safety_count = 0;
    'line_reader: for line in get_lines(path)?
    {
        let line_vals = collect_line_vals(line?)?;
        let mut direction = ValueDirection::None;
        let mut unsafety_count = 0;
        for i in 1..line_vals.len()
        {
            let last = line_vals[i - 1];
            let cur = line_vals[i];
            match direction
            {
                ValueDirection::None =>
                {
                    if cur > last
                    {
                        direction = ValueDirection::Ascending;
                    }

                    if cur < last
                    {
                        direction = ValueDirection::Descending
                    }
                }
                ValueDirection::Ascending =>
                {
                    if cur < last
                    {
                        unsafety_count += 1;
                    }
                }
                ValueDirection::Descending =>
                {
                    if cur > last
                    {
                        unsafety_count += 1;
                    }
                }
            }

            let diff = cur.abs_diff(last);
            if !(1..=3).contains(&diff)
            {
                unsafety_count += 1;
            }
        }
        if unsafety_count <= 1
        {
            safety_count += 1;
        }
    }

    Ok(safety_count)
}
