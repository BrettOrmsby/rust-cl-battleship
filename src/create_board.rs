use crate::terminal_utils::print_center;

use super::ship::{Point, Ship, ShipDirection, ShipKind};
use super::terminal_utils::create_colored_grid;
use console::{Key, Style, Term};
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
            term.clear_last_lines(23);
            render(&term, &ships);
            let key = term.read_key();
            match key {
                Ok(Key::Char(' ')) => rotate_ship(&mut ships),
                Ok(Key::ArrowUp) => move_ship_up(&mut ships),
                Ok(Key::ArrowDown) => move_ship_down(&mut ships),
                Ok(Key::ArrowLeft) => move_ship_left(&mut ships),
                Ok(Key::ArrowRight) => move_ship_right(&mut ships),
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
    term.clear_last_lines(23);
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
                    if last_ship_points.contains(&&Point(j, i)) && ship_points.contains(&&Point(j, i))
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

/// Rotates the last ship clockwise if possible
fn rotate_ship(ships: &mut [Ship]) {
    if let Some(last_ship) = ships.last() {
        let new_direction = match last_ship.direction {
            ShipDirection::Down => ShipDirection::Left,
            ShipDirection::Left => ShipDirection::Up,
            ShipDirection::Up => ShipDirection::Right,
            ShipDirection::Right => ShipDirection::Down,
        };
        if Ship::can_exist(&last_ship.kind, last_ship.x, last_ship.y, &new_direction) {
            if let Some(last_ship) = ships.last_mut() {
                last_ship.direction = new_direction;
                last_ship.reset_points();
            }
        }
    }
}

/// Moves the last ship up if possible
fn move_ship_up(ships: &mut [Ship]) {
    if let Some(last_ship) = ships.last() {
        if last_ship.y == 0 {
            return;
        }
        if Ship::can_exist(
            &last_ship.kind,
            last_ship.x,
            last_ship.y - 1,
            &last_ship.direction,
        ) {
            if let Some(last_ship) = ships.last_mut() {
                last_ship.y -= 1;
                last_ship.reset_points();
            }
        }
    }
}

/// Moves the last ship down if possible
fn move_ship_down(ships: &mut [Ship]) {
    if let Some(last_ship) = ships.last() {
        if Ship::can_exist(
            &last_ship.kind,
            last_ship.x,
            last_ship.y + 1,
            &last_ship.direction,
        ) {
            if let Some(last_ship) = ships.last_mut() {
                last_ship.y += 1;
                last_ship.reset_points();
            }
        }
    }
}

/// Moves the last ship left if possible
fn move_ship_left(ships: &mut [Ship]) {
    if let Some(last_ship) = ships.last() {
        if last_ship.x == 0 {
            return;
        }
        if Ship::can_exist(
            &last_ship.kind,
            last_ship.x - 1,
            last_ship.y,
            &last_ship.direction,
        ) {
            if let Some(last_ship) = ships.last_mut() {
                last_ship.x -= 1;
                last_ship.reset_points();
            }
        }
    }
}

/// Moves the last ship right if possible
fn move_ship_right(ships: &mut [Ship]) {
    if let Some(last_ship) = ships.last() {
        if Ship::can_exist(
            &last_ship.kind,
            last_ship.x + 1,
            last_ship.y,
            &last_ship.direction,
        ) {
            if let Some(last_ship) = ships.last_mut() {
                last_ship.x += 1;
                last_ship.reset_points();
            }
        }
    }
}
