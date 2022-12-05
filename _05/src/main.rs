fn main() {
    // let input = std::fs::read_to_string("input").expect("Failed to read input!");

    // let mut line_iter = input.split('\n');

    // let raw_stacks: Vec<Vec<_>> = line_iter
    //     .by_ref()
    //     .map_while(|line| {
    //         if !line.starts_with(" 1") {
    //             Some(
    //                 line.chars()
    //                     .enumerate()
    //                     .filter_map(|(i, cr)| if i % 4 == 1 { Some(cr) } else { None })
    //                     .collect(),
    //             )
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // let mut stacks: Vec<Vec<_>> = Vec::new();
    // for layer in raw_stacks.into_iter().rev() {
    //     if layer.len() > stacks.len() {
    //         stacks.resize_with(layer.len(), Default::default);
    //     }
    //     for (i, cr) in layer.into_iter().enumerate() {
    //         if cr != ' ' {
    //             stacks[i].push(cr);
    //         }
    //     }
    // }

    // for line in line_iter.filter(|line| !line.is_empty()) {
    //     let mut command_iter = line.split(' ');
    //     // skip move
    //     command_iter.next();

    //     let amount = command_iter.next().unwrap().parse::<usize>().unwrap();

    //     // skip from
    //     command_iter.next();

    //     let source = command_iter.next().unwrap().parse::<usize>().unwrap() - 1;

    //     // skip to
    //     command_iter.next();

    //     let dest: usize = command_iter.next().unwrap().parse::<usize>().unwrap() - 1;

    //     for _ in 0..amount {
    //         let cr = stacks[source].pop().unwrap();
    //         stacks[dest].push(cr);
    //     }
    // }

    // for stack in stacks {
    //     print!("{}", stack.last().unwrap());
    // }
    // println!();

    use std::cell::RefCell;

    let input = std::fs::read_to_string("input").expect("Failed to read input!");

    let mut line_iter = input.split('\n');

    let raw_stacks: Vec<Vec<_>> = line_iter
        .by_ref()
        .map_while(|line| {
            if !line.starts_with(" 1") {
                Some(
                    line.chars()
                        .enumerate()
                        .filter_map(|(i, cr)| if i % 4 == 1 { Some(cr) } else { None })
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();

    let mut stacks: Vec<RefCell<Vec<_>>> = Vec::new();
    for layer in raw_stacks.into_iter().rev() {
        if layer.len() > stacks.len() {
            stacks.resize_with(layer.len(), Default::default);
        }
        for (i, cr) in layer.into_iter().enumerate() {
            if cr != ' ' {
                stacks[i].borrow_mut().push(cr);
            }
        }
    }

    for line in line_iter.filter(|line| !line.is_empty()) {
        let mut command_iter = line.split(' ');
        // skip move
        command_iter.next();

        let amount = command_iter.next().unwrap().parse::<usize>().unwrap();

        // skip from
        command_iter.next();

        let source = command_iter.next().unwrap().parse::<usize>().unwrap() - 1;

        // skip to
        command_iter.next();

        let dest: usize = command_iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let bottom_pop = stacks[source].borrow().len() - amount;
        stacks[dest]
            .borrow_mut()
            .extend_from_slice(&stacks[source].borrow()[bottom_pop..]);
        stacks[source].borrow_mut().resize(bottom_pop, '!');
    }

    for stack in stacks {
        print!("{}", stack.borrow().last().unwrap());
        // print!("{:?}", stack.borrow());
        // println!();
    }
    println!();
}
