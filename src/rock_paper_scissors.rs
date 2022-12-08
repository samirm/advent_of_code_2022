
fn main() {
    let file_as_string: String = std::fs::read_to_string("data/rock_paper_scissors.txt").expect("no such file").parse().expect("could not parse");
    let lines: Vec<&str> = file_as_string.lines().collect();

    let mut plays: Vec<(&str, &str)> = vec!();

    for line in lines {
        plays.push(line.split_at(1))
    }

    const WIN: i32 = 6;
    const DRAW: i32 = 3;
    const LOSS: i32 = 0;
    let mut score = 0;

    for play in plays {
        let opponent = play.0;
        let player = play.1.trim();

        if opponent == "A" { //Rock
            if player == "X" { //Rock
                score += DRAW + PlayerMoveDictionary::X as i32
            } else if player == "Y" { //Paper
                score += WIN + PlayerMoveDictionary::Y as i32
            } else if player == "Z" { //Scissors
                score += LOSS + PlayerMoveDictionary::Z as i32
            }
        } else if opponent == "B" { //Paper
            if player == "X" { //Rock
                score += LOSS + PlayerMoveDictionary::X as i32
            } else if player == "Y" { //Paper
                score += DRAW + PlayerMoveDictionary::Y as i32
            } else if player == "Z" { //Scissors
                score += WIN + PlayerMoveDictionary::Z as i32
            }
        } else if opponent == "C" { //Scissors
            if player == "X" { //Rock
                score += WIN + PlayerMoveDictionary::X as i32
            } else if player == "Y" { //Paper
                score += LOSS + PlayerMoveDictionary::Y as i32
            } else if player == "Z" { //Scissors
                score += DRAW + PlayerMoveDictionary::Z as i32
            }
        }
    }

    println!("{}", score);
}

// #[derive(EnumString)]
enum PlayerMoveDictionary {
    X = 1,
    Y = 2,
    Z = 3
}
