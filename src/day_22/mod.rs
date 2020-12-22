use std::collections::VecDeque;
use std::fs;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day_22.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

fn part_1(input: Game) -> u64 {
    let result = run_game(input);
    score_player(result.winning_player())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day_22.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

fn part_2(input: Game) -> u64 {
    let result = run_recursive_game(input);

    match result {
        GameResult::PlayerOneWin(game) => score_player(game.winning_player()),
        GameResult::PlayerTwoWin(game) => score_player(game.winning_player()),
    }
}

fn parse_player_input(player: &str) -> VecDeque<u32> {
    player.split('\n').skip(1).map(|i| i.parse().unwrap()).collect()
}

fn parse_input(input: &str) -> Game {
    let players: Vec<_> = input.split("\n\n").map(parse_player_input).collect();
    Game {
        players,
        previous_states_this_game: vec![],
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<VecDeque<u32>>,
    previous_states_this_game: Vec<Vec<VecDeque<u32>>>,
}

impl Game {
    fn winning_player(&self) -> &VecDeque<u32> {
        self.players.iter().find(|player| !player.is_empty()).unwrap()
    }

    fn check_for_previous_state(&self) -> bool {
        let most_recently_added = self.previous_states_this_game.last();
        match most_recently_added {
            Some(most_recent_state) => self
                .previous_states_this_game
                .iter()
                .take(self.previous_states_this_game.len() - 1)
                .any(|state| *state == *most_recent_state),
            None => false,
        }
    }
}

fn score_player(player: &VecDeque<u32>) -> u64 {
    player
        .iter()
        .rev()
        .enumerate()
        .map(|(i, val)| (((i + 1) as u64) * (*val as u64)))
        .sum()
}

fn run_game(game: Game) -> Game {
    let mut player_1 = game.players[0].clone();
    let mut player_2 = game.players[1].clone();

    while !player_1.is_empty() && !player_2.is_empty() {
        let (top_1, top_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        if top_1 > top_2 {
            player_1.push_back(top_1);
            player_1.push_back(top_2);
        } else {
            player_2.push_back(top_2);
            player_2.push_back(top_1);
        };
    }
    let players = vec![player_1, player_2];
    Game {
        players,
        previous_states_this_game: vec![],
    }
}

#[derive(Debug)]
enum GameResult {
    PlayerOneWin(Game),
    PlayerTwoWin(Game),
}

fn run_recursive_game(mut game: Game) -> GameResult {
    let mut player_1 = game.players[0].clone();
    let mut player_2 = game.players[1].clone();
    while !player_1.is_empty() && !player_2.is_empty() {
        if game.check_for_previous_state() {
            // println!("Exiting as we have seen this state before: {:#?}", vec![player_1.clone(), player_2.clone()]);
            return GameResult::PlayerOneWin(game);
        } else {
            game.previous_states_this_game
                .push(vec![player_1.clone(), player_2.clone()])
        }
        let (top_1, top_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        // println!("Player 1 plays: {}, player_2 plays: {}", top_1, top_2);
        if player_1.len() >= (top_1 as usize) && player_2.len() >= (top_2 as usize) {
            // println!("Recursing");
            let new_player_1 = player_1.iter().cloned().take(top_1 as usize).collect();
            let new_player_2 = player_2.iter().cloned().take(top_2 as usize).collect();
            let this_round_winner = run_recursive_game(Game {
                players: vec![new_player_1, new_player_2],
                previous_states_this_game: vec![],
            });
            match this_round_winner {
                GameResult::PlayerOneWin(_) => {
                    // println!("Player one won");
                    player_1.push_back(top_1);
                    player_1.push_back(top_2);
                }
                GameResult::PlayerTwoWin(_) => {
                    // println!("Player two won");
                    player_2.push_back(top_2);
                    player_2.push_back(top_1);
                }
            }
        } else {
            // println!("Playing plain");
            if top_1 > top_2 {
                // println!("Player one won");
                player_1.push_back(top_1);
                player_1.push_back(top_2);
            } else {
                // println!("Player two won");
                player_2.push_back(top_2);
                player_2.push_back(top_1);
            };
        }
    }

    let players = vec![player_1.clone(), player_2];
    let game_to_return = Game {
        players,
        previous_states_this_game: vec![],
    };
    // println!("Reached the end of this game where player {} won", if !player_1.is_empty() { "1" } else { "2" });
    if !player_1.is_empty() {
        GameResult::PlayerOneWin(game_to_return)
    } else {
        GameResult::PlayerTwoWin(game_to_return)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_game() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day22").unwrap());
        assert_eq!(part_1(input), 306);
    }

    #[test]
    fn test_running_recursive_game() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day22").unwrap());
        assert_eq!(part_2(input), 291);
    }
}
