use std::ops::RangeInclusive;

fn main() {
    // let input = std::fs::read_to_string("input").expect("Failed to read input!");

    // let overlap_count = input
    //     .trim()
    //     .split('\n')
    //     .map(|pair| pair.split_once(',').unwrap())
    //     .filter(|(left, right)| {
    //         fn parse_range(s: &str) -> RangeInclusive<u32> {
    //             s.split_once('-')
    //                 .map(|(start, end)| start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap())
    //                 .unwrap()
    //         }

    //         let left = parse_range(left);
    //         let right = parse_range(right);

    //         fn contains_range_incl(
    //             range: &RangeInclusive<u32>,
    //             other: &RangeInclusive<u32>,
    //         ) -> bool {
    //             range.contains(other.start()) && range.contains(other.end())
    //         }

    //         contains_range_incl(&left, &right) || contains_range_incl(&right, &left)
    //     })
    //     .count();

    // println!("{}", overlap_count);

    let input = std::fs::read_to_string("input").expect("Failed to read input!");

    let overlap_count = input
        .trim()
        .split('\n')
        .map(|pair| pair.split_once(',').unwrap())
        .filter(|(left, right)| {
            fn parse_range(s: &str) -> RangeInclusive<u32> {
                s.split_once('-')
                    .map(|(start, end)| start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap())
                    .unwrap()
            }

            let left = parse_range(left);
            let right = parse_range(right);

            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count();

    println!("{}", overlap_count);
}
