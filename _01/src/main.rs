use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let text = read_to_string("input").expect("Failed to read input!");

    // let max_elf = text
    //     .trim()
    //     .split("\n\n")
    //     .map(|elf| {
    //         elf.split('\n')
    //             .map(|cstr| cstr.parse::<u32>().expect("Parsing failed"))
    //             .sum::<u32>()
    //     })
    //     .max()
    //     .unwrap();

    // println!("{:?}", max_elf);

    let mut prio_q: BinaryHeap<_> = text
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cstr| cstr.parse::<u32>().expect("Parsing failed"))
                .sum::<u32>()
        })
        .collect();

    let mut sum = 0;
    for _ in 0..3 {
        sum += prio_q.pop().unwrap();
    }

    println!("{:?}", sum);
}
