use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TileType {
    Empty,
    Cookie,
    Milk,
    Wall,
}
impl Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Empty => write!(f, "⬛"),
            TileType::Cookie => write!(f, "🍪"),
            TileType::Milk => write!(f, "🥛"),
            TileType::Wall => write!(f, "⬜"),
        }
    }
}
impl From<&Player> for TileType {
    fn from(value: &Player) -> Self {
        match value {
            Player::Cookie => Self::Cookie,
            Player::Milk => Self::Milk
        }
    }
}
#[derive(Debug)]
pub(crate) enum Player {
    Cookie,
    Milk,
}
impl TryFrom<&TileType> for Player {
    type Error = ();
    fn try_from(t: &TileType) -> Result<Self, Self::Error> {
        match t {
            TileType::Cookie => Ok(Player::Cookie),
            TileType::Milk => Ok(Player::Milk),
            _ => Err(())
        }
    }
}

impl TryFrom<&str> for Player {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cookie" => Ok(Player::Cookie),
            "milk" => Ok(Player::Milk),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub(crate) enum GameState {
    Win(Player),
    NoWin,
    Pending,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Grid {
    grid: [TileType; 16], // every 4 elements makes up a row
    last_placed: Option<usize>,
    empty_slots_left: usize,
}
impl Default for Grid {
    fn default() -> Self {
        Self {
            grid: [TileType::Empty; 16],
            last_placed: None,
            empty_slots_left: 16,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();
        for row in self.grid.chunks_exact(4) {
            let [a, b, c, d] = <&[TileType; 4]>::try_from(row).expect("4 items");
            result.push(format!("⬜{}{}{}{}⬜", a, b, c, d));
        }
        result.push("⬜⬜⬜⬜⬜⬜\n".to_string());
        write!(f, "{}", result.join("\n"))?;
        match self.check_winner() {
            GameState::Win(player) => {
                write!(f, "{} wins!\n", TileType::from(&player))?;
            }
            GameState::NoWin => {
                write!(f, "No winner.\n")?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Grid {
    // todo clean up this mess
    pub fn check_winner(&self) -> GameState {
        if let Some(last_position) = self.last_placed {
            let central_slot = &self.grid[last_position];
            let row_next_slot = &self.grid[(last_position + 1) % 4 * ((last_position % 4) + 1)];
            let row_next_2_slots = &self.grid[(last_position + 2) % 4 * ((last_position % 4) + 1)];
            let row_previous_slot = &self.grid[(last_position + 3) % 4 * ((last_position % 4) + 1)];
            let column_next_slot = &self.grid[(last_position + 4) % 16];
            let column_next_2_slot = &self.grid[(last_position + 8) % 16];
            let column_previous_slot = &self.grid[(last_position + 12) % 16];
            if let Ok(player) = Player::try_from(central_slot) {
                tracing::info!("Checking row with elements: {:#?}, {:#?}, {:#?}, {:#?}", central_slot, row_next_slot, row_next_2_slots, row_previous_slot);
                if central_slot == row_next_slot && central_slot == row_next_2_slots && central_slot == row_previous_slot {
                    return GameState::Win(player);
                }
                tracing::info!("Checking column with elements: {:#?}, {:#?}, {:#?}, {:#?}", central_slot, column_next_slot, column_next_2_slot, column_previous_slot);
                if central_slot == column_next_slot && central_slot == column_next_2_slot && central_slot == column_previous_slot {
                    return GameState::Win(player);
                }
            }

            let diagonal_slot_1 = &self.grid[0];
            let diagonal_slot_2 = &self.grid[5];
            let diagonal_slot_3 = &self.grid[10];
            let diagonal_slot_4 = &self.grid[15];
            if let Ok(player) = Player::try_from(diagonal_slot_1) {
                if diagonal_slot_1 == diagonal_slot_2 && diagonal_slot_1 == diagonal_slot_3 && diagonal_slot_1 == diagonal_slot_4 {
                    return GameState::Win(player);
                }
            }


            let off_diagonal_1 = &self.grid[3];
            let off_diagonal_2 = &self.grid[6];
            let off_diagonal_3 = &self.grid[9];
            let off_diagonal_4 = &self.grid[12];
            if let Ok(player) = Player::try_from(off_diagonal_1) {
                if off_diagonal_1 == off_diagonal_2 && off_diagonal_1 == off_diagonal_3 && off_diagonal_1 == off_diagonal_4 {
                    return GameState::Win(player);
                }
            }
        }
        if self.empty_slots_left == 0 {
            GameState::NoWin
        } else {
            GameState::Pending
        }
    }

    pub fn place(&mut self, tile: TileType, column: usize) -> Result<(), ()> {
        for depth in (column..=column + 12).rev().step_by(4) {
            tracing::info!("Checking tile {:#?}", self.grid[depth]);
            if let TileType::Empty = self.grid[depth] {
                tracing::info!("Found available spot for column {} at depth {}", column, depth);
                self.grid[depth] = tile;
                self.last_placed = Some(depth);
                self.empty_slots_left -= 1;
                return Ok(());
            }
        }
        tracing::info!("No available spot for column {}", column);
        Err(())
    }
}