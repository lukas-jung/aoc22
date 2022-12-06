use std::collections::HashSet;

fn main() {
    // let window_size = 4;
    let window_size = 14;

    let input = std::fs::read_to_string("input").expect("Failed to read input!");

    let start_pos = input
        .trim()
        .as_bytes()
        .windows(window_size)
        .map(|w| HashSet::<u8>::from_iter(w.iter().copied()).len())
        .enumerate()
        .filter_map(|(index, different_chars)| {
            if different_chars == window_size {
                Some(index + window_size)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    println!("{}", start_pos);
}
