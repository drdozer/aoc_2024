use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    // We've got an input that is a wordsearch grid.
    // We will treat it as bytes rather than as chars, as it appears to be restricted to ASCII.
    // We're going to assume all lines have the same length.
    // The line length is the number of bytes per line.
    // The row length is the number of bytes including the newline.
    // The total number of lines is found by dividing the total length by the row length,
    // taking into account the possible absense of a newline at the very end.

    let input = input.as_bytes();
    let line_len = input.iter().take_while(|&&b| b != b'\n').count();
    let row_len = line_len + 1;

    #[cfg(debug_assertions)]
    {
        let lines = (input.len() + 1) / row_len;
        let overhang = (input.len() + 1) % row_len;
        assert!(
            overhang == 0 || overhang == 1,
            "Not expecting a trailing fragment: line_len: {}, row_len: {}, lines: {}, overhang: {}",
            line_len,
            row_len,
            lines,
            overhang
        );
    }

    // println!(
    //     "line length: {}, lines: {}, overhang: {}",
    //     line_len, lines, overhang
    // );

    // We can avoid reversing input data by matching against the reverse target.
    let xmas = b"XMAS";
    let samx = b"SAMX";
    let mut xmas_count = 0;

    // The horizontal search is very simple.
    for e in 3..input.len() {
        let horizontal = unsafe {
            [
                *input.get_unchecked(e - 3),
                *input.get_unchecked(e - 2),
                *input.get_unchecked(e - 1),
                *input.get_unchecked(e),
            ]
        };
        // if e == 3 || e == input.len() - 1 {
        //     println!(
        //         "e: {:?}, horizontal: {:?}",
        //         e,
        //         std::str::from_utf8(&horizontal)
        //     );
        // }
        xmas_count += (&horizontal == xmas) as usize;
        xmas_count += (&horizontal == samx) as usize;
    }

    // All other searches involve pulling bytes from multiple lines.
    // The vertical search needs to start from where XMAS directly down from the start would end.
    for e in (3 * row_len)..input.len() {
        let vertical = unsafe {
            [
                *input.get_unchecked(e - 3 * row_len),
                *input.get_unchecked(e - 2 * row_len),
                *input.get_unchecked(e - 1 * row_len),
                *input.get_unchecked(e),
            ]
        };
        // if e == 3 * row_len || e == input.len() - 1 {
        //     println!("e: {:?}, vertical: {:?}", e, std::str::from_utf8(&vertical));
        // }
        xmas_count += (&vertical == xmas) as usize;
        xmas_count += (&vertical == samx) as usize;
    }

    // The diagonal search needs to start 3 chars further to the right.
    for e in (3 * row_len + 3)..input.len() {
        let leading_diagonal = unsafe {
            [
                *input.get_unchecked(e - 3 * row_len - 3),
                *input.get_unchecked(e - 2 * row_len - 2),
                *input.get_unchecked(e - 1 * row_len - 1),
                *input.get_unchecked(e),
            ]
        };

        // if e == 3 * row_len + 3 || e == input.len() - 1 {
        //     println!(
        //         "e: {:?}, diagonal: {:?}",
        //         e,
        //         std::str::from_utf8(&leading_diagonal)
        //     );
        // }
        xmas_count += (&leading_diagonal == xmas) as usize;
        xmas_count += (&leading_diagonal == samx) as usize;

        let trailing_diagonal = unsafe {
            [
                *input.get_unchecked(e - 3 * row_len),
                *input.get_unchecked(e - 2 * row_len - 1),
                *input.get_unchecked(e - 1 * row_len - 2),
                *input.get_unchecked(e - 3),
            ]
        };

        // if e == 3 * row_len + 3 || e == input.len() - 1 {
        //     println!(
        //         "e: {:?}, diagonal: {:?}",
        //         e,
        //         std::str::from_utf8(&trailing_diagonal)
        //     );
        // }
        xmas_count += (&trailing_diagonal == xmas) as usize;
        xmas_count += (&trailing_diagonal == samx) as usize;
    }

    xmas_count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let line_len = input.iter().take_while(|&&b| b != b'\n').count();
    let row_len = line_len + 1;

    #[cfg(debug_assertions)]
    {
        let lines = (input.len() + 1) / row_len;
        let overhang = (input.len() + 1) % row_len;
        debug_assert!(
            overhang == 0 || overhang == 1,
            "Not expecting a trailing fragment: line_len: {}, row_len: {}, lines: {}, overhang: {}",
            line_len,
            row_len,
            lines,
            overhang
        );
    }

    // This time, we're looking for all occurances of:
    //
    // M.S
    // .A.
    // M.S
    //
    // All rotations and flips are considered valid hits.
    // One approach is to find all occurances of A, and then check the surronding diagonals.
    // We're going to again load the patch of input into a byte array and then compare it with the 4 possible targets.

    let right = b"MSAMS";
    let left = b"SMASM";
    let down = b"MMASS";
    let up = b"SSAMM";
    let mut xmas_count = 0;
    for i in row_len + 1..(input.len() - row_len - 1) {
        let patch = unsafe {
            [
                *input.get_unchecked(i - row_len - 1),
                *input.get_unchecked(i - row_len + 1),
                *input.get_unchecked(i),
                *input.get_unchecked(i + row_len - 1),
                *input.get_unchecked(i + row_len + 1),
            ]
        };
        // if i == row_len + 1 || i == input.len() - row_len - 1 {
        // println!("i: {:?}, patch: {:?}", i, std::str::from_utf8(&patch));
        // }
        xmas_count += (&patch == right) as usize;
        xmas_count += (&patch == left) as usize;
        xmas_count += (&patch == down) as usize;
        xmas_count += (&patch == up) as usize;
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let example = indoc! {
            "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"
        };
        assert_eq!(part1(example), 18);
    }

    #[test]
    fn part1_horizontal() {
        let example = indoc! {
            "XMASXMAS
            XMASXMAS
            XMASXMAS"
        };
        assert_eq!(part1(example), 6);

        let example = indoc! {
            "SAMXSAMX
            SAMXSAMX
            SAMXSAMX"
        };
        assert_eq!(part1(example), 6);
    }

    #[test]
    fn part1_vertical() {
        let example = indoc! {
            "XXX
            MMM
            AAA
            SSS"
        };
        assert_eq!(part1(example), 3);

        let example = indoc! {
            "SSS
            AAA
            MMM
            XXX"
        };
        assert_eq!(part1(example), 3);
    }

    #[test]
    fn part1_diagonal_right() {
        let example = indoc! {
            "XXXooo
            oMMMoo
            ooAAAo
            oooSSS"
        };
        assert_eq!(part1(example), 3);
    }

    #[test]
    fn part1_diagonal_left() {
        let example = indoc! {
            "oooXXX
            ooMMMo
            oAAAoo
            SSSooo"
        };
        assert_eq!(part1(example), 3);
    }

    #[test]
    fn part2_example() {
        let example = indoc! {
            "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"
        };
        assert_eq!(part2(example), 9);
    }
}
