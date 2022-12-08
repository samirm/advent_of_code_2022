use std::collections::HashSet;

fn main() {
    let file_as_string: String = std::fs::read_to_string("data/rucksack.txt").expect("no such file").parse().expect("could not parse");
    let lines: Vec<&str> = file_as_string.lines().collect();

    let mut rucksacks: Vec<(HashSet<char>, HashSet<char>)> = vec!();

    for line in lines {
        let tup = line.split_at(line.len()/2);
        let left_ruck = HashSet::from_iter(tup.0.chars());
        let right_ruck = HashSet::from_iter(tup.1.chars());
        rucksacks.push((left_ruck, right_ruck));
    }
    // println!("{:?}", rucksacks);

    let mut total = 0;
    for sack in rucksacks {
        let mut c = sack.0.intersection(&sack.1);
        let d = *c.next().unwrap() as i32;
        // println!("d: {}", d);
        if d < 97 {
            total += (d-38)
        } else {
            total += (d-96)
        }
        // println!("total: {}", total);
        // print_type_of(c.next().unwrap());
        // println!("{:?}", *c.next().unwrap() as u8);
        // println!("{:?}", c.next().unwrap());
    }

    println!("\npart1: {}", total);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}