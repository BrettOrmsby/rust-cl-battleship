/// A simple point
#[derive(PartialEq, Debug, Clone)]
pub struct Point(pub u8, pub u8);

/// The direction the ship is "traveling"
#[derive(Debug, Clone, Copy)]
pub enum ShipDirection {
    Up,
    Down,
    Left,
    Right,
}

/// The type of ship
#[derive(Debug, Clone, Copy)]
pub enum ShipKind {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipKind {
    /// Length of the ship
    pub fn get_len(&self) -> u8 {
        match self {
            ShipKind::Carrier => 5,
            ShipKind::Battleship => 4,
            ShipKind::Cruiser => 3,
            ShipKind::Submarine => 3,
            ShipKind::Destroyer => 2,
        }
    }
    /// Name of the ship
    pub fn get_name(&self) -> &'static str {
        match self {
            ShipKind::Carrier => "Carrier",
            ShipKind::Battleship => "Battleship",
            ShipKind::Cruiser => "Cruiser",
            ShipKind::Submarine => "Submarine",
            ShipKind::Destroyer => "Destroyer",
        }
    }
}
/// Stores ships
#[derive(Debug)]
pub struct Ship {
    pub x: u8,
    pub y: u8,
    pub direction: ShipDirection,
    pub kind: ShipKind,

    pub points: Vec<Point>,
    pub hit_points: Vec<Point>,
}

impl Ship {
    /// Try to create a ship
    pub fn build(
        kind: ShipKind,
        x: u8,
        y: u8,
        direction: ShipDirection,
    ) -> Result<Self, &'static str> {
        if Self::can_exist(&kind, x, y, &direction) {
            let mut ship = Self {     
                x, y, direction,           
                kind,
                points: vec![],
                hit_points: vec![]
            };
            ship.reset_points();
            Ok(ship)
        } else {
            Err("Out of bounds.")
        }
    }

    /// Resets the points based on x, y, and direction
    pub fn reset_points(&mut self) {
        if Self::can_exist(&self.kind, self.x, self.y, &self.direction) {
            let length = self.kind.get_len() - 1; // Do not count the head
        let (x_min, x_max, y_min, y_max) = match &self.direction {
            ShipDirection::Down => (self.x, self.x, self.y - length, self.y),
            ShipDirection::Up => (self.x, self.x, self.y, self.y + length),
            ShipDirection::Left => (self.x, self.x + length, self.y, self.y),
            ShipDirection::Right => (self.x - length, self.x, self.y, self.y),
        };

        self.points = (x_min..=x_max)
            .flat_map(|x| (y_min..=y_max).map(|y| Point(x, y)).collect::<Vec<Point>>()).collect();
    }
    }

    /// Check if a ship can be created from the parameters
    pub fn can_exist(kind: &ShipKind, x: u8, y: u8, direction: &ShipDirection) -> bool {
        let length = kind.get_len() - 1; // Do not count the head

        let tail = match direction {
            ShipDirection::Down if y < length => return false,
            ShipDirection::Down => y - length,
            ShipDirection::Up => y + length,
            ShipDirection::Left => x + length,
            ShipDirection::Right if x < length => return false,
            ShipDirection::Right => x - length,
        };
        let bounds = [x, y, tail];
        if bounds.iter().any(|pos| *pos > 9) {
            return false;
        }
        true
    }

    /// Check weather two ships intercept
    pub fn does_intercept(&self, ship: &Ship) -> bool {
        self.points
            .iter()
            .any(|point| ship.points.contains(point))
    }

    /// Check weather the ship is hit
    pub fn is_hit_by(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    /// Records a hit on a ship and returns weather the ship was sunk
    pub fn hit(&mut self, point: Point) {
        if self.is_hit_by(&point) {
            self.hit_points.push(point);
        }
    }

    /// Checks weather the ship is sunk
    pub fn is_sunk(&self) -> bool {
        self.hit_points.len() >= self.kind.get_len() as usize
    }
    
    /// Rotates the last ship clockwise if possible
pub fn rotate(&mut self) {
        let new_direction = match self.direction {
            ShipDirection::Down => ShipDirection::Left,
            ShipDirection::Left => ShipDirection::Up,
            ShipDirection::Up => ShipDirection::Right,
            ShipDirection::Right => ShipDirection::Down,
        };
        if Ship::can_exist(&self.kind, self.x, self.y, &new_direction) {
                self.direction = new_direction;
                self.reset_points();
        }
    }

/// Moves the last ship up if possible
pub fn move_up(&mut self) {
        if self.y == 0 {
            return;
        }
        if Ship::can_exist(
            &self.kind,
            self.x,
            self.y - 1,
            &self.direction,
        ) {
                self.y -= 1;
                self.reset_points();
    }
}

/// Moves the last ship down if possible
pub fn move_down(&mut self) {
        if Ship::can_exist(
            &self.kind,
            self.x,
            self.y + 1,
            &self.direction,
        ) {
                self.y += 1;
                self.reset_points();
    }
}

/// Moves the last ship left if possible
pub fn move_left(&mut self) {
        if self.x == 0 {
            return;
        }
        if Ship::can_exist(
            &self.kind,
            self.x - 1,
            self.y,
            &self.direction,
        ) {
        
                self.x -= 1;
                self.reset_points();
        }
}

/// Moves the last ship right if possible
pub fn move_right(&mut self) {
        if Ship::can_exist(
            &self.kind,
            self.x + 1,
            self.y,
            &self.direction,
        ) {
                self.x += 1;
                self.reset_points();
      
    }
}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_make_ships_at_boundaries() {
        // Top left corner
        Ship::build(ShipKind::Destroyer, 1, 1, ShipDirection::Down).unwrap();
        Ship::build(ShipKind::Destroyer, 1, 1, ShipDirection::Left).unwrap();
        Ship::build(ShipKind::Destroyer, 0, 0, ShipDirection::Up).unwrap();
        Ship::build(ShipKind::Destroyer, 0, 0, ShipDirection::Left).unwrap();

        // Bottom right corner
        Ship::build(ShipKind::Destroyer, 8, 8, ShipDirection::Up).unwrap();
        Ship::build(ShipKind::Destroyer, 8, 8, ShipDirection::Right).unwrap();
        Ship::build(ShipKind::Destroyer, 9, 9, ShipDirection::Down).unwrap();
        Ship::build(ShipKind::Destroyer, 9, 9, ShipDirection::Right).unwrap();
    }

    #[test]
    fn ships_intercepts() {
        let ship1 = Ship::build(ShipKind::Carrier, 3, 1, ShipDirection::Up).unwrap();
        let ship2 = Ship::build(ShipKind::Battleship, 2, 3, ShipDirection::Left).unwrap();
        let ship3 = Ship::build(ShipKind::Destroyer, 2, 4, ShipDirection::Down).unwrap();
        let ship4 = Ship::build(ShipKind::Submarine, 3, 3, ShipDirection::Right).unwrap();

        assert!(ship1.does_intercept(&ship2));
        assert!(ship1.does_intercept(&ship4));
        assert!(ship2.does_intercept(&ship3));
        assert!(ship2.does_intercept(&ship4));
        assert!(ship3.does_intercept(&ship4));

        // And this should not intercept
        assert!(!ship1.does_intercept(&ship3));
    }

    #[test]
    fn ships_get_hit() {
        let mut ship1 = Ship::build(ShipKind::Carrier, 3, 1, ShipDirection::Up).unwrap();
        // No hits
        assert!(!ship1.is_hit_by(&Point(3, 0)));
        assert!(!ship1.is_hit_by(&Point(3, 6)));
        assert!(!ship1.is_hit_by(&Point(2, 3)));
        assert!(!ship1.is_hit_by(&Point(4, 3)));

        // Real hits
        assert!(ship1.is_hit_by(&Point(3, 1)));
        assert!(!ship1.is_sunk());
        assert!(ship1.is_hit_by(&Point(3, 2)));
        assert!(!ship1.is_sunk());
        assert!(ship1.is_hit_by(&Point(3, 3)));
        assert!(!ship1.is_sunk());
        assert!(ship1.is_hit_by(&Point(3, 4)));
        assert!(!ship1.is_sunk());
        assert!(ship1.is_hit_by(&Point(3, 5)));
        assert!(ship1.is_sunk());
    }
}
