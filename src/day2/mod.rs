#[allow(dead_code)]
pub fn total_score(input: &str, strategy: StrategyType) -> i32 {
    games(input, strategy).iter().map(|game| game.score()).sum()
}

pub enum StrategyType {
    MapToPlay,
    MapToOutcome,
}

fn games(input: &str, strategy: StrategyType) -> Vec<Game> {
    Reader {
        input: input,
        strategy_type: strategy,
    }
    .games()
}

trait Strategy {
    fn you(&self, opponent: &Play, value: &str) -> Play;
}

struct MapToPlay {}

impl Strategy for MapToPlay {
    fn you(&self, _opponent: &Play, value: &str) -> Play {
        match value {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissers,
            _ => panic!("You '#{value}' is invalid"),
        }
    }
}

struct MapToOutcome {}

impl Strategy for MapToOutcome {
    fn you(&self, opponent: &Play, value: &str) -> Play {
        match value {
            "X" => match opponent {
                // Loose
                Play::Rock => Play::Scissers,
                Play::Paper => Play::Rock,
                Play::Scissers => Play::Paper,
            },
            "Y" => match opponent {
                // Draw
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissers => Play::Scissers,
            },
            "Z" => match opponent {
                // Win
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissers,
                Play::Scissers => Play::Rock,
            },
            _ => panic!("You '#{value}' is invalid"),
        }
    }
}

struct Reader<'a> {
    input: &'a str,
    strategy_type: StrategyType,
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
        let you = self.strategy().you(&opponent, col2);
        Game {
            opponent: opponent,
            you: you,
        }
    }

    fn strategy(&self) -> Box<dyn Strategy> {
        match self.strategy_type {
            StrategyType::MapToPlay => Box::new(MapToPlay {}),
            StrategyType::MapToOutcome => Box::new(MapToOutcome {}),
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
    use super::StrategyType;
    use std::fs;

    #[test]
    fn one_game() {
        let input = "A Y";
        assert_eq!(total_score(&input, StrategyType::MapToPlay), 8);
    }

    #[test]
    fn three_game() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(total_score(&input, StrategyType::MapToPlay), 15);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day2/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_score(&input, StrategyType::MapToPlay), 9177);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day2/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_score(&input, StrategyType::MapToOutcome), 12111);
    }
}
