use std::collections::HashSet;

fn main() {
    // let input = std::fs::read_to_string("input").expect("Failed to read input!");

    // let mut comp1_set = HashSet::new();

    // let sum: u32 = input
    //     .trim()
    //     .split('\n')
    //     .map(|rs| rs.split_at(rs.len() / 2))
    //     .map(|(comp1, comp2)| {
    //         // clear and extend instead of assignment to reuse allocated memory
    //         comp1_set.clear();
    //         comp1_set.extend(comp1.bytes());

    //         // find doubled_item
    //         comp2.bytes().find(|item| comp1_set.contains(item)).unwrap()
    //     })
    //     .map(|doubled| {
    //         // 'move' uppercase letters to the end of lowercase letters.
    //         // then subtract ascii char before 'a' to convert.
    //         (if doubled.is_ascii_uppercase() {
    //             doubled.to_ascii_lowercase() + 26
    //         } else {
    //             doubled
    //         } - (b'a' - 1)) as u32
    //     })
    //     .sum();

    // println!("{}", sum);

    use itertools::Itertools;

    let input = std::fs::read_to_string("input").expect("Failed to read input!");
    let mut sets = [HashSet::new(), HashSet::new(), HashSet::new()];

    let sum: u32 = input
        .trim()
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|group| {
            // clear and extend sets instead of assignment to reuse allocated memory
            for (i, elf) in group.enumerate() {
                sets[i].clear();
                sets[i].extend(elf.bytes())
            }

            sets[0]
                .intersection(&sets[1])
                .filter(|elem| sets[2].contains(elem))
                .next()
                .copied()
                .unwrap()
        })
        .map(|badge| {
            // 'move' uppercase letters to the end of lowercase letters.
            // then subtract ascii char before 'a' to convert.
            (if badge.is_ascii_uppercase() {
                badge.to_ascii_lowercase() + 26
            } else {
                badge
            } - (b'a' - 1)) as u32
        })
        .sum();

    println!("{}", sum);
}
