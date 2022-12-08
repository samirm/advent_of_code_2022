
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
            if player == "X" {
                score += LOSS + PlayerMoveDictionary::Scissors as i32
            } else if player == "Y" {
                score += DRAW + PlayerMoveDictionary::Rock as i32
            } else if player == "Z" {
                score += WIN + PlayerMoveDictionary::Paper as i32
            }
        } else if opponent == "B" { //Paper
            if player == "X" {
                score += LOSS + PlayerMoveDictionary::Rock as i32
            } else if player == "Y" {
                score += DRAW + PlayerMoveDictionary::Paper as i32
            } else if player == "Z" {
                score += WIN + PlayerMoveDictionary::Scissors as i32
            }
        } else if opponent == "C" { //Scissors
            if player == "X" {
                score += LOSS + PlayerMoveDictionary::Paper as i32
            } else if player == "Y" {
                score += DRAW + PlayerMoveDictionary::Scissors as i32
            } else if player == "Z" {
                score += WIN + PlayerMoveDictionary::Rock as i32
            }
        }
    }

    println!("{}", score);
}

// #[derive(EnumString)]
enum PlayerMoveDictionary {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
