#[allow(dead_code)]
pub fn total_score(input: &str) -> i32 {
    games(input).iter().map(|game| game.score()).sum()
}

fn games(input: &str) -> Vec<Game> {
    Reader { input: input }.games()
}

struct Reader<'a> {
    input: &'a str,
}

impl<'a> Reader<'a> {
    fn games(&self) -> Vec<Game> {
        self.input.split("\n").map(|x| self.game(x)).collect()
    }

    fn game(&self, input: &str) -> Game {
        let cols: Vec<&str> = input.split(" ").collect();
        let (col1, col2) = (cols[0], cols[1]);
        let opponent = match col1 {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissers,
            _ => panic!("Opponent '#{col1}' is invalid"),
        };
        let you = match col2 {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissers,
            _ => panic!("You '#{col2}' is invalid"),
        };
        Game {
            opponent: opponent,
            you: you,
        }
    }
}

#[allow(dead_code)]
pub fn total_score_2(input: &str) -> i32 {
    games_2(input).iter().map(|game| game.score()).sum()
}

fn games_2(input: &str) -> Vec<Game> {
    Reader2 { input: input }.games()
}

struct Reader2<'a> {
    input: &'a str,
}

impl<'a> Reader2<'a> {
    fn games(&self) -> Vec<Game> {
        self.input.split("\n").map(|x| self.game(x)).collect()
    }

    fn game(&self, input: &str) -> Game {
        let cols: Vec<&str> = input.split(" ").collect();
        let (col1, col2) = (cols[0], cols[1]);
        let opponent = match col1 {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissers,
            _ => panic!("Opponent '#{col1}' is invalid"),
        };
        let you = match col2 {
            "X" => match opponent {
                Play::Rock => Play::Scissers,
                Play::Paper => Play::Rock,
                Play::Scissers => Play::Paper,
            },
            "Y" => match opponent {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissers => Play::Scissers,
            },
            "Z" => match opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissers,
                Play::Scissers => Play::Rock,
            },
            _ => panic!("You '#{col2}' is invalid"),
        };
        Game {
            opponent: opponent,
            you: you,
        }
    }
}

#[derive(PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissers,
}

struct Game {
    opponent: Play,
    you: Play,
}

impl Game {
    fn score(&self) -> i32 {
        let mut result = 0;
        result += self.you_score();
        result += self.even_score();
        result += self.win_score();
        result
    }

    fn you_score(&self) -> i32 {
        match self.you {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissers => 3,
        }
    }

    fn even_score(&self) -> i32 {
        if self.you == self.opponent {
            3
        } else {
            0
        }
    }

    fn win_score(&self) -> i32 {
        let win = match self.you {
            Play::Rock => self.opponent == Play::Scissers,
            Play::Paper => self.opponent == Play::Rock,
            Play::Scissers => self.opponent == Play::Paper,
        };
        if win {
            6
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::total_score;
    use super::total_score_2;
    use std::fs;

    #[test]
    fn one_game() {
        let input = "A Y";
        assert_eq!(total_score(&input), 8);
    }

    #[test]
    fn three_game() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(total_score(&input), 15);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day2/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_score(&input), 9177);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day2/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_score_2(&input), 12111);
    }
}
