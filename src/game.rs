use std::vec;

use console::{Alignment, Style, Term, style};
use rand::{Rng, rng};

use crate::{
    create_board::{self, GameBoard, GridState},
    ship::Point,
    terminal_utils::{self, create_colored_grid},
};

pub struct Game {
    player_board: GameBoard,
    bot_board: GameBoard,
    is_game_over: bool,
}

impl Game {
    /// Create a game
    pub fn new() -> Self {
        Self {
            player_board: create_board::start(),
            bot_board: create_board::generate_game_board(),
            is_game_over: false,
        }
    }
    /// Start the game
    pub fn start_game(&mut self) {
        let term = Term::buffered_stdout();
        self.render(&term);
        term.flush();
        loop {
            let target = get_target(&term, &self.bot_board);
            let first_message = self.update_hit(false, target);
            if self.is_game_over {
                term.clear_last_lines(26);
                term.write_line(&first_message);
                term.write_line(&format!("{} You won!", style("  Win ").on_yellow().bold(),));
                self.render(&term);
                term.flush();
                break;
            }
            let bot_target = gen_bot_target(&self.player_board);
            let second_message = self.update_hit(true, bot_target);
            if self.is_game_over {
                term.clear_last_lines(26);
                term.write_line(&first_message);
                term.write_line(&second_message);
                term.write_line(&format!("{} You lose!", style(" Loss ").on_black().bold(),));
                self.render(&term);
                term.flush();
                break;
            }
            term.clear_last_lines(26);
            term.write_line(&first_message);
            term.write_line(&second_message);
            self.render(&term);
            term.flush();
        }
    }

    /// Render grids and messages
    fn render(&self, term: &Term) {
        let player_grid = generate_grid(&self.player_board, true);
        let bot_grid = generate_grid(&self.bot_board, false);
        let grids = terminal_utils::join(bot_grid, player_grid, 2);
        let grid_width = 46;
        let grid_labels = format!(
            "\n{}{}",
            style(
                console::pad_str("Target Board", grid_width, Alignment::Center, None,).to_string()
            )
            .bold(),
            style(
                console::pad_str("Your Board", grid_width - 1, Alignment::Center, None,)
                    .to_string()
            )
            .bold()
        );
        term.write_line(&grid_labels);
        term.write_line(&grids);
        term.flush();
    }

    /// Updates information about the hit
    fn update_hit(&mut self, is_player_board_hit: bool, target: Point) -> String {
        let hit_board = if is_player_board_hit {
            &mut self.player_board
        } else {
            &mut self.bot_board
        };
        let hit_index = hit_board
            .ships
            .iter()
            .position(|ship| ship.is_hit_by(&target));
        if let Some(hit_index) = hit_index {
            let hit_ship = &mut hit_board.ships[hit_index];
            hit_board.board[target.1 as usize][target.0 as usize] = GridState::Hit;
            let mut hit_message = String::new();
            if is_player_board_hit {
                hit_message += &format!(
                    "{} The Admiral hit your {}.",
                    style("  Hit ").on_red().bold(),
                    hit_ship.kind.get_name()
                )
            } else {
                hit_message += &format!(
                    "{} You hit the {}.",
                    style("  Hit ").on_green().bold(),
                    hit_ship.kind.get_name()
                )
            };
            if hit_ship.hit() {
                if is_player_board_hit {
                    hit_message += &format!(
                        "\n{} The Admiral sunk your {}.",
                        style(" Sunk ").on_red().bold(),
                        hit_ship.kind.get_name()
                    )
                } else {
                    hit_message += &format!(
                        "\n{} You sunk the {}.",
                        style(" Sunk ").on_green().bold(),
                        hit_ship.kind.get_name()
                    )
                };
                hit_board.ships_left -= 1;
                if hit_board.ships_left == 0 {
                    self.is_game_over = true;
                };
            }
            hit_message
        } else {
            hit_board.board[target.1 as usize][target.0 as usize] = GridState::Miss;
            if is_player_board_hit {
                format!("{} The Admiral missed.", style(" Miss ").on_white().bold())
            } else {
                format!("{} You missed.", style(" Miss ").on_white().bold())
            }
        }
    }
}

/// Read user input to determine their target
fn get_target(term: &Term, target_board: &GameBoard) -> Point {
    term.write_line("Enter the striking coordinates: ");
    'valid_input: loop {
        term.flush();
        let input = term.read_line();
        if input.is_err() {
            continue;
        }
        let input = input.unwrap();
        let mut char_pos: Option<char> = None;
        let mut number_pos: Option<u8> = None;
        for char in input.chars() {
            match char {
                'A'..='J' | 'a'..='j' => {
                    if char_pos.is_some() {
                        term.clear_last_lines(2);
                        term.write_line(&format!("{} Multiple Letter Coordinates Provided. Enter new striking coordinates: ", style(" Error ").on_red().bold()));
                        term.flush();
                        continue 'valid_input;
                    } else {
                        char_pos = Some(char);
                    }
                }
                '0'..='9' => {
                    if let Some(digit) = number_pos {
                        if digit == 1 && char == '0' {
                            number_pos = Some(10);
                        } else {
                            term.clear_last_lines(2);
                            term.write_line(&format!("{} Multiple Numeric Coordinates Provided. Enter new striking coordinates: ", style(" Error ").on_red().bold()));
                            term.flush();
                            continue 'valid_input;
                        }
                    } else if char != '0' {
                        number_pos = Some(char as u8 - b'0');
                    } else {
                        term.clear_last_lines(2);
                        term.write_line(&format!("{} Multiple Numeric Coordinates Provided. Enter new striking coordinates: ", style(" Error ").on_red().bold()));
                        term.flush();
                        continue 'valid_input;
                    }
                }
                _ => (),
            }
        }
        if char_pos.is_none() {
            term.clear_last_lines(2);
            term.write_line(&format!(
                "{} No Letter Coordinates Provided. Enter new striking coordinates: ",
                style(" Error ").on_red().bold()
            ));
            term.flush();
            continue;
        } else if number_pos.is_none() {
            term.clear_last_lines(2);
            term.write_line(&format!(
                "{} No Numeric Coordinates Provided. Enter new striking coordinates: ",
                style(" Error ").on_red().bold()
            ));
            term.flush();
            continue;
        }

        let y = char_pos.unwrap().to_ascii_uppercase() as u8 - b'A';
        let point = Point(number_pos.unwrap() - 1, y);
        if target_board.board[point.1 as usize][point.0 as usize] != GridState::Blank {
            term.clear_last_lines(2);
            term.write_line(&format!(
                "{} Duplicate Strike. Enter new striking coordinates: ",
                style(" Error ").on_red().bold()
            ));
            term.flush();
            continue;
        }
        term.clear_last_lines(2);
        term.flush();
        return point;
    }
}

/// Randomly generate the target for the bot
// TODO: prioritize ships with two spots hit to continue linearly
fn gen_bot_target(target_board: &GameBoard) -> Point {
    let mut possible_positions: Vec<Point> = vec![];
    let mut recommended_positions: Vec<Point> = vec![];
    for i in 0..9 {
        for j in 0..9 {
            match target_board.board[j][i] {
                GridState::Blank => possible_positions.push(Point(i as u8, j as u8)),
                GridState::Hit => {
                    // Prioritize hitting near ships that are hit but not sunk
                    let is_ship_floating = target_board.ships.iter().any(|ship| {
                        ship.get_points().contains(&Point(i as u8, j as u8))
                            && ship.number_hits != ship.kind.get_len()
                    });
                    if !is_ship_floating {
                        continue;
                    }
                    // Look Up
                    if j > 0 && target_board.board[j - 1][i] == GridState::Blank {
                        recommended_positions.push(Point(i as u8, (j - 1) as u8));
                    }
                    // Look Down
                    if j < 9 && target_board.board[j + 1][i] == GridState::Blank {
                        recommended_positions.push(Point(i as u8, (j + 1) as u8));
                    }
                    // Look Left
                    if i > 0 && target_board.board[j][i - 1] == GridState::Blank {
                        recommended_positions.push(Point((i - 1) as u8, j as u8));
                    }
                    // Look Right
                    if i < 9 && target_board.board[j][i + 1] == GridState::Blank {
                        recommended_positions.push(Point((i + 1) as u8, j as u8));
                    }
                }
                _ => (),
            }
        }
    }

    if !recommended_positions.is_empty() {
        recommended_positions[rng().random_range(0..recommended_positions.len())].clone()
    } else {
        possible_positions[rng().random_range(0..possible_positions.len())].clone()
    }
}

/// Generates a grid
fn generate_grid(game_board: &GameBoard, show_ships: bool) -> String {
    let ship_points: Vec<Point> = game_board
        .ships
        .iter()
        .flat_map(|ship| ship.get_points())
        .collect();
    let coloured_grid: Vec<Vec<Style>> = (0..10)
        .map(|i| {
            (0..10)
                .map(|j| match game_board.board[i][j] {
                    GridState::Miss => Style::new().white(),
                    GridState::Hit => Style::new().red(),
                    GridState::Blank => {
                        if ship_points.contains(&Point(j as u8, i as u8)) && show_ships {
                            if (i + j) % 2 == 0 {
                                Style::new().black().bold()
                            } else {
                                Style::new().black()
                            }
                        } else if (i + j) % 2 == 0 {
                            Style::new().blue().bold()
                        } else {
                            Style::new().blue()
                        }
                    }
                })
                .collect()
        })
        .collect();
    create_colored_grid(&coloured_grid)
}
