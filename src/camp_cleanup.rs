use std::collections::HashSet;

fn main() {
    let file_as_string: String = std::fs::read_to_string("data/camp.txt").expect("no such file").parse().expect("could not parse");
    let lines: Vec<&str> = file_as_string.lines().collect();

    let mut total = 0;

    for line in lines {
        let this_pair = line.split(",").collect::<Vec<&str>>();

        let first_elf = this_pair[0].split("-")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();
        let second_elf = this_pair[1].split("-")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();

        // if first_elf[1] - first_elf[0] <= second_elf[1] - second_elf[0] {
        //     if first_elf[0] >= second_elf[0] && first_elf[1] <= second_elf[1] {
        //         total += 1
        //     } else {
        //         continue
        //     }
        // } else if second_elf[1] - second_elf[0] <= first_elf[1] - first_elf[0] {
        //     if second_elf[0] >= first_elf[0] && second_elf[1] <= first_elf[1] {
        //         total += 1
        //     } else {
        //         continue
        //     }
        // }

        let mut set1 = HashSet::new();

        for i in first_elf[0]..first_elf[1]+1 {
            // println!("{}", i);
            set1.insert(i);
        }
        for i in second_elf[0]..second_elf[1]+1 {
            if set1.contains(&i) {
                total += 1;
                break
            }
        }
    }
    println!("part2: {:?}", total);
}
