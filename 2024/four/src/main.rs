use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
};

use color_eyre::Result;
const WORD_SEARCH_SEQUENCE: [char; 4] = ['X', 'M', 'A', 'S'];
fn main() -> Result<()>
{
    let xmas_frequency = perform_word_search(Path::new("./src/input.txt"))?;
    println!("The total value for part one is {xmas_frequency}.");
    let xmas_frequency = perform_x_mas_search(Path::new("./src/input.txt"))?;
    println!("The total value for part one is {xmas_frequency}.");
    Ok(())
}

fn load_file(path: &Path) -> Result<Vec<Vec<char>>>
{
    let input_file = File::open(path)?;
    let buf_reader = BufReader::new(input_file);

    Ok(buf_reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>())
}

fn perform_word_search(path: &Path) -> Result<usize>
{
    let matrix = load_file(path)?;
    let mut above_available = false;
    let mut below_available = true;

    let mut xmas_count = 0;
    let expected_chars = ('M', 'A', 'S');

    for i in 0..matrix.len()
    {
        if i == WORD_SEARCH_SEQUENCE.len() - 1
        {
            above_available = true;
        }

        let mut left_available = false;
        let mut right_available = true;

        for j in 0..matrix[i].len()
        {
            if j == WORD_SEARCH_SEQUENCE.len() - 1
            {
                left_available = true;
            }

            if matrix[i][j] == 'X'
            {
                // Check characters that are directly above
                // and above to the left and right.
                if above_available
                {
                    // Check line directly above.
                    if (matrix[i - 1][j], matrix[i - 2][j], matrix[i - 3][j]) == expected_chars
                    {
                        xmas_count += 1;
                    }

                    // Check if left and above.
                    if left_available
                        && (
                            matrix[i - 1][j - 1],
                            matrix[i - 2][j - 2],
                            matrix[i - 3][j - 3],
                        ) == expected_chars
                    {
                        xmas_count += 1;
                    }

                    // Check if right and above.
                    if right_available
                        && (
                            matrix[i - 1][j + 1],
                            matrix[i - 2][j + 2],
                            matrix[i - 3][j + 3],
                        ) == expected_chars
                    {
                        xmas_count += 1;
                    }
                }

                // Check directly left.
                if left_available
                    && (matrix[i][j - 1], matrix[i][j - 2], matrix[i][j - 3]) == expected_chars
                {
                    xmas_count += 1;
                }

                // Check directly right.
                if right_available
                    && (matrix[i][j + 1], matrix[i][j + 2], matrix[i][j + 3]) == expected_chars
                {
                    xmas_count += 1;
                }

                if below_available
                {
                    // Check directly below.
                    if (matrix[i + 1][j], matrix[i + 2][j], matrix[i + 3][j]) == expected_chars
                    {
                        xmas_count += 1;
                    }

                    // Check below and to the left.
                    if left_available
                        && (
                            matrix[i + 1][j - 1],
                            matrix[i + 2][j - 2],
                            matrix[i + 3][j - 3],
                        ) == expected_chars
                    {
                        xmas_count += 1;
                    }

                    // Check below and to the right.
                    if right_available
                        && (
                            matrix[i + 1][j + 1],
                            matrix[i + 2][j + 2],
                            matrix[i + 3][j + 3],
                        ) == expected_chars
                    {
                        xmas_count += 1;
                    }
                }
            }

            // This is the last loop iteration where right
            // is valid, so we set the bool to false before
            // the next iteration.
            if j == matrix[i].len() - (WORD_SEARCH_SEQUENCE.len())
            {
                right_available = false;
            }
        }

        // This is the last loop iteration where below is
        // valid, so we set the bool to false before the
        // next iteration.
        if i == matrix.len() - (WORD_SEARCH_SEQUENCE.len())
        {
            below_available = false;
        }
    }
    Ok(xmas_count)
}

fn perform_x_mas_search(path: &Path) -> Result<usize>
{
    let matrix = load_file(path)?;

    let mut x_mas_count = 0;

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == 'M'
            {}
        }
    }

    Ok(x_mas_count)
}
