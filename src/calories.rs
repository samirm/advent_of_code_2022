
#[derive(Debug)]
struct Elf {
    id: i32,
    total_calories: i32
}

fn main() {
    let mut elves: Vec<Elf> = vec![];
    let mut this_id = 0;

    let string: String = std::fs::read_to_string("data/calories.txt").expect("no such file").parse().expect("could not parse");
    // println!("{}", string);

    let lines: Vec<&str> = string.lines().collect();
    // println!("{:?}", lines);
    let mut total = 0;
    for line in lines {
        if !line.is_empty() {
            total += line.parse::<i32>().unwrap();
        } else {
            this_id += 1;
            let elf = Elf { id: this_id, total_calories: total };
            // println!("{:?}", elf);
            elves.push(elf);
            total = 0;
        }
    }

    elves.sort_unstable_by_key(|e| e.total_calories);
    println!("part1: {:?}", elves.last());

    let mut last3 = 0;
    for elf in elves.iter().rev().take(3) {
        last3 += elf.total_calories;
    }
    println!("part2: {}", last3)
}
