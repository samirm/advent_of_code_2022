
fn main() {
    let file_as_string: String = std::fs::read_to_string("data/crates.txt").expect("no such file").parse().expect("could not parse");
    let lines: Vec<&str> = file_as_string.lines().collect();
    let mut moves: Vec<Vec<&str>> = vec![];

    for line in lines {
        moves.push(line.split_whitespace().collect());
    }

        // [N] [G]                     [Q]
        // [H] [B]         [B] [R]     [H]
        // [S] [N]     [Q] [M] [T]     [Z]
        // [J] [T]     [R] [V] [H]     [R] [S]
        // [F] [Q]     [W] [T] [V] [J] [V] [M]
        // [W] [P] [V] [S] [F] [B] [Q] [J] [H]
        // [T] [R] [Q] [B] [D] [D] [B] [N] [N]
        // [D] [H] [L] [N] [N] [M] [D] [D] [B]
        //  1   2   3   4   5   6   7   8   9

    let mut stack1 = vec!["D", "T", "W", "F", "J", "S", "H", "N"];
    let mut stack2 = vec!["H", "R", "P", "Q", "T", "N", "B", "G"];
    let mut stack3 = vec!["L", "Q", "V"];
    let mut stack4 = vec!["N", "B", "S", "W", "R", "Q"];
    let mut stack5 = vec!["N", "D", "F", "T", "V", "M", "B"];
    let mut stack6 = vec!["M", "D", "B", "V", "H", "T", "R"];
    let mut stack7 = vec!["D", "B", "Q", "J"];
    let mut stack8 = vec!["D", "N", "J", "V", "R", "Z", "H", "Q"];
    let mut stack9 = vec!["B", "N", "H", "M", "S"];
    let mut list_of_stacks = vec![stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9];

    for this_move in moves {
        let take_size = this_move[1].parse::<usize>().unwrap();
        let dest = this_move[5].parse::<usize>().unwrap() - 1;
        let src = this_move[3].parse::<usize>().unwrap() - 1;
        // println!("src: {:?}", src);
        // println!("dest: {:?}", dest);
        let mut src_stack = &list_of_stacks[src];
        let mut src_stack_size = list_of_stacks[src].len();
        println!("src: {:?}", src_stack);
        let mut mini_stack = &mut src_stack[src_stack_size-take_size..src_stack_size].to_vec();
        println!("mini: {:?}", mini_stack);
        list_of_stacks[dest].append(mini_stack);
        println!("dest: {:?}", list_of_stacks[dest]);
        for x in 0..take_size {
            list_of_stacks[src].pop();
        }
        // println!("src: {:?}", src_stack);
    }

    for mut stack in list_of_stacks {
        print!("{}", stack.pop().unwrap());
    }
}