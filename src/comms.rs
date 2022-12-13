use std::collections::HashSet;

fn main() {
    let file_as_string: String = std::fs::read_to_string("data/comms.txt").expect("no such file").parse().expect("could not parse");
    let lines: Vec<&str> = file_as_string.lines().collect();
    let chars: Vec<char> = lines[0].chars().collect();

    let mut start_of_packet = HashSet::new();
    let mut index = 0;

    for i in 4..chars.len() {
        start_of_packet = HashSet::from_iter(&chars[i-4..i]);
        println!("{:?}", start_of_packet);
        if start_of_packet.len() == 4 {
            println!("yes");
            index = i;
            break
        }
    }
    println!("{}", index);
}
