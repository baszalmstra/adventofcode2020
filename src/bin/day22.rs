use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct GameState {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

enum RoundResult {
    Winner(bool, VecDeque<usize>),
    NoWinner(GameState),
}

impl GameState {
    fn recursive_combat_turn(
        mut self,
        previously_played: &mut HashMap<GameState, (bool, VecDeque<usize>)>,
    ) -> RoundResult {
        let player1 = self.player1.pop_front().unwrap();
        let player2 = self.player2.pop_front().unwrap();

        let player1_is_winner = if self.player1.len() < player1 || self.player2.len() < player2 {
            player1 > player2
        } else {
            let sub_game = GameState {
                player1: self.player1.iter().take(player1).copied().collect(),
                player2: self.player2.iter().take(player2).copied().collect(),
            };

            let result = sub_game.clone().play_recursive(previously_played);
            let player_1_won = result.0;
            previously_played.insert(sub_game, result);
            player_1_won
        };

        if player1_is_winner {
            self.player1.push_back(player1);
            self.player1.push_back(player2);
            if self.player2.is_empty() {
                return RoundResult::Winner(true, self.player1);
            }
        } else {
            self.player2.push_back(player2);
            self.player2.push_back(player1);
            if self.player1.is_empty() {
                return RoundResult::Winner(false, self.player2);
            }
        }
        RoundResult::NoWinner(self)
    }

    fn combat_turn(mut self) -> RoundResult {
        let player1 = self.player1.pop_front().unwrap();
        let player2 = self.player2.pop_front().unwrap();
        if player1 > player2 {
            self.player1.push_back(player1);
            self.player1.push_back(player2);
            if self.player2.is_empty() {
                return RoundResult::Winner(true, self.player1);
            }
        } else {
            self.player2.push_back(player2);
            self.player2.push_back(player1);
            if self.player1.is_empty() {
                return RoundResult::Winner(false, self.player2);
            }
        }
        RoundResult::NoWinner(self)
    }

    fn play(mut self) -> VecDeque<usize> {
        loop {
            match self.combat_turn() {
                RoundResult::Winner(_, winner) => break winner,
                RoundResult::NoWinner(state) => {
                    self = state;
                }
            }
        }
    }

    fn play_recursive(
        mut self,
        previously_played: &mut HashMap<GameState, (bool, VecDeque<usize>)>,
    ) -> (bool, VecDeque<usize>) {
        let mut previous_rounds = HashSet::new();
        loop {
            if let Some(result) = previously_played.get(&self) {
                return result.clone();
            }

            if previous_rounds.contains(&self) {
                break (true, self.player1);
            }

            previous_rounds.insert(self.clone());

            match { self.recursive_combat_turn(previously_played) } {
                RoundResult::Winner(is_player_1, winner) => break (is_player_1, winner),
                RoundResult::NoWinner(state) => {
                    self = state;
                }
            }
        }
    }
}

fn parse(input: &str) -> GameState {
    fn parse_hand<'a>(lines: &mut impl Iterator<Item = &'a str>) -> VecDeque<usize> {
        lines.next().unwrap();
        let mut result = VecDeque::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            result.push_back(line.parse().unwrap())
        }
        result
    }

    let mut lines = input.lines();
    GameState {
        player1: parse_hand(&mut lines),
        player2: parse_hand(&mut lines),
    }
}

fn calculate_hand_score(hand: &VecDeque<usize>) -> usize {
    hand.iter()
        .enumerate()
        .map(|(i, value)| (hand.len() - i) * *value)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day22/input").unwrap();
    let initial_game_state = parse(&input);

    let mut previously_played = HashMap::new();

    println!(
        "Solution 1: {}",
        calculate_hand_score(&initial_game_state.clone().play())
    );
    println!(
        "Solution 2: {}",
        calculate_hand_score(&initial_game_state.play_recursive(&mut previously_played).1)
    );
}
