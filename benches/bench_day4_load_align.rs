use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchark_copy_delete_newlines(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");

    c.bench_function("copy without newlines", |b| {
        b.iter(|| copy_without_newlines(black_box(input)))
    });
}

pub fn benchmark_horizontal_search(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");

    c.bench_function("horizontal search 4 bytes", |b| {
        b.iter(|| horizontal_search_bytewise(black_box(input)))
    });

    c.bench_function("horizontal search 1 byte", |b| {
        b.iter(|| horizontal_search_savebytes(black_box(input)))
    });
}

fn copy_without_newlines(input: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    for c in input.as_bytes().iter().filter(|&&b| b != b'\n') {
        output.push(*c);
    }
    output
}

fn horizontal_search_bytewise(input: &str) -> usize {
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

    let xmas = b"XMAS";
    let samx = b"SAMX";
    let mut xmas_count = 0;

    // initialize the first 3 bytes of the horizontal search
    let mut horizontal = [0u8; 4];
    unsafe {
        horizontal[1] = *input.get_unchecked(0);
        horizontal[2] = *input.get_unchecked(1);
        horizontal[3] = *input.get_unchecked(2);
    }

    for e in 3..input.len() {
        horizontal[0] = horizontal[1];
        horizontal[1] = horizontal[2];
        horizontal[2] = horizontal[3];
        horizontal[3] = unsafe { *input.get_unchecked(e) };
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

    xmas_count
}

fn horizontal_search_savebytes(input: &str) -> usize {
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

    xmas_count
}

criterion_group!(
    benches,
    benchark_copy_delete_newlines,
    benchmark_horizontal_search,
);
criterion_main!(benches);
