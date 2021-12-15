#[derive(Clone, Debug)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

// Direction determines which cell the tree grows into next, given its current position in the 2d grid.
#[derive(Copy, Clone, Debug)]
pub enum Direction {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TurnDirection {
  Left,
  Right,
}

impl Direction {
  pub fn is_moving_north(&self) -> bool {
    use Direction::*;
    match self {
      North => true,
      NorthEast => true,
      East => false,
      SouthEast => false,
      South => false,
      SouthWest => false,
      West => false,
      NorthWest => true,
    }
  }

  pub fn is_moving_west(&self) -> bool {
    use Direction::*;
    match self {
      North => false,
      NorthEast => false,
      East => false,
      SouthEast => false,
      South => false,
      SouthWest => false,
      West => true,
      NorthWest => true,
    }
  }

  pub fn is_moving_east(&self) -> bool {
    use Direction::*;
    match self {
      North => false,
      NorthEast => true,
      East => true,
      SouthEast => false,
      South => false,
      SouthWest => false,
      West => false,
      NorthWest => false,
    }
  }

  pub fn is_moving_horizontally(&self) -> bool {
    use Direction::*;
    match self {
      North => false,
      NorthEast => false,
      East => true,
      SouthEast => false,
      South => false,
      SouthWest => false,
      West => true,
      NorthWest => false,
    }
  }

  pub fn turn(&self, turning: TurnDirection) -> Self {
    use Direction::*;
    use TurnDirection::*;
    match self {
      North => {
        if turning == Left {
          NorthWest
        } else {
          NorthEast
        }
      }
      NorthEast => {
        if turning == Left {
          North
        } else {
          East
        }
      }
      East => {
        if turning == Left {
          NorthEast
        } else {
          SouthEast
        }
      }
      SouthEast => {
        if turning == Left {
          East
        } else {
          South
        }
      }
      South => {
        if turning == Left {
          SouthEast
        } else {
          SouthWest
        }
      }
      SouthWest => {
        if turning == Left {
          South
        } else {
          West
        }
      }
      West => {
        if turning == Left {
          SouthWest
        } else {
          NorthWest
        }
      }
      NorthWest => {
        if turning == Left {
          West
        } else {
          North
        }
      }
    }
  }
}
