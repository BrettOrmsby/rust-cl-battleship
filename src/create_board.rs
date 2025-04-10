use crate::terminal_utils::print_center;

use super::ship::{Point, Ship, ShipDirection, ShipKind};
use super::terminal_utils::create_colored_grid;
use console::{style, Key, Style, Term};
use rand::{self, Rng};
#[derive(Clone, Copy, PartialEq)]
pub enum GridState {
    Blank,
    Hit,
    Miss,
}
pub struct GameBoard {
    pub board: [[GridState; 10]; 10],
    pub ships: Vec<Ship>,
    pub ships_left: u8,
}

impl GameBoard {
    fn build(ships: Vec<Ship>) -> Self {
        Self {
            board: [[GridState::Blank; 10]; 10],
            ships,
            ships_left: 5,
        }
    }
}

// TODO: handle r for restart, or possibly a,1 for ship positions
/// Sets up the users board
pub fn start() -> GameBoard {
    let term = Term::buffered_stdout();
    let mut ships: Vec<Ship> = vec![];
    render(&term, &ships);
    term.write_line("\n");
    [
        ShipKind::Carrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Submarine,
        ShipKind::Destroyer,
    ]
    .iter()
    .for_each(|ship_kind| {
        // Create the ship
        let ship = Ship::build(*ship_kind, 0, 0, ShipDirection::Up).unwrap();
        ships.push(ship);
        loop {
            term.clear_last_lines(25);
            print_center(&term, &format!("{}", style("Set Up").bold()));
            print_center(&term, &format!("Use your {} keys to move the ship, {} to rotate, and {} to set its position.", style("Arrow").bold(), style("Space").bold(), style("Enter").bold()));
            render(&term, &ships);
            let key = term.read_key();
            match key {
                Ok(Key::Char(' ')) => ships.last_mut().unwrap().rotate(),
                Ok(Key::ArrowUp) | Ok(Key::Char('w'))|  Ok(Key::Char('W')) => ships.last_mut().unwrap().move_up(),
                Ok(Key::ArrowDown) | Ok(Key::Char('s'))|  Ok(Key::Char('S')) => ships.last_mut().unwrap().move_down(),
                Ok(Key::ArrowLeft)| Ok(Key::Char('a'))|  Ok(Key::Char('A')) => ships.last_mut().unwrap().move_left(),
                Ok(Key::ArrowRight) | Ok(Key::Char('d'))|  Ok(Key::Char('D'))=> ships.last_mut().unwrap().move_right(),
                Ok(Key::Enter) => {
                    if let Some(last_ship) = ships.last() {
                        let does_collide = ships.iter().any(|ship| {
                            ship as *const _ != last_ship as *const _
                                && ship.does_intercept(last_ship)
                        });
                        if !does_collide {
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    });
    term.clear_last_lines(25);
    term.flush();

    GameBoard::build(ships)
}

/// Create a random ship layout
pub fn generate_game_board() -> GameBoard {
    let mut rng = rand::rng();
    let ships: Vec<Ship> = [
        ShipKind::Carrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Submarine,
        ShipKind::Destroyer,
    ]
    .iter()
    .fold(vec![], |mut ships, ship_kind| {
        // Generate random direction
        let direction = match rng.random_range(0..4) {
            0 => ShipDirection::Down,
            1 => ShipDirection::Up,
            3 => ShipDirection::Left,
            _ => ShipDirection::Right,
        };
        // Now go through every position and see if a ship can be placed there
        let mut possible_positions: Vec<Point> = vec![];
        for i in 0..9 {
            for j in 0..9 {
                if Ship::can_exist(ship_kind, j, i, &direction) {
                    let new_ship = Ship::build(*ship_kind, j, i, direction)
                        .expect("Somehow, the ship can't exist");
                    if !ships.iter().any(|ship| ship.does_intercept(&new_ship)) {
                        possible_positions.push(Point(j, i))
                    }
                }
            }
        }

        let random_position = &possible_positions[rng.random_range(0..possible_positions.len())];
        ships.push(
            Ship::build(*ship_kind, random_position.0, random_position.1, direction)
                .expect("Somehow, the ship can't exist"),
        );
        ships
    });
    GameBoard::build(ships)
}

/// Renders a battleship grid
pub fn render(term: &Term, ships: &[Ship]) {
    let last_ship_index = if !ships.is_empty() {
        ships.len() - 1
    } else {
        0
    }; // Avoid overflow error
    let ship_points: Vec<&Point> = ships
        .iter()
        .take(last_ship_index)
        .flat_map(|ship| &ship.points)
        .collect();
    let last_ship_points: &Vec<Point> = if let Some(last_ship) = ships.last() {
        &last_ship.points
    } else {
        &vec![]
    };
    let coloured_grid: Vec<Vec<Style>> = (0..10)
        .map(|i| {
            (0..10)
                .map(|j| {
                    if last_ship_points.contains(&Point(j, i)) && ship_points.contains(&&Point(j, i))
                    {
                        Style::new().red()
                    } else if last_ship_points.contains(&Point(j, i)) {
                        Style::new().green()
                    } else if ship_points.contains(&&Point(j, i)) {
                        if (i + j) % 2 == 0 {
                            Style::new().black().bright()
                        } else {
                            Style::new().black()
                        }
                    } else if (i + j) % 2 == 0 {
                        Style::new().blue().bold()
                    } else {
                        Style::new().blue()
                    }
                })
                .collect()
        })
        .collect();
    let grid = create_colored_grid(&coloured_grid);
    print_center(term, &grid);
    term.flush();
}
