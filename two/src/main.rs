use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
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
        read_safety_input(Path::new("./src/input.txt")).expect("Failed calculating safety total.");
    println!("The safety value for this input is: {safety_value}");
}

fn read_safety_input(path: &Path) -> Result<usize>
{
    let mut safety_count = 0_usize;

    let input_file = File::open(path)?;
    let buf_reader = BufReader::new(input_file);

    'line_reader: for line in buf_reader.lines()
    {
        // Parse the line into a vec of usize vals.
        let unwrapped_line = line?;
        let line_vals = unwrapped_line
            .split(INPUT_LINE_DELIMITER)
            .collect::<Vec<&str>>()
            .iter()
            .map(|val| usize::from_str(val).unwrap())
            .collect::<Vec<usize>>();

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
