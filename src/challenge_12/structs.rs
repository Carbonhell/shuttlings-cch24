use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
enum TileType {
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

#[derive(Debug, Copy, Clone)]
pub(crate) struct Grid {
    grid: [TileType; 16], // every 4 elements makes up a row
}
impl Default for Grid {
    fn default() -> Self {
        Self {
            grid: [TileType::Empty; 16],
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
        write!(f, "{}", result.join("\n"))
    }
}