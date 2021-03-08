#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct AxialCoord((i32, i32));

impl AxialCoord {
    pub const fn new(row: i32, column: i32) -> Self {
        Self((row, column))
    }
    pub const fn row(self) -> i32 {
        self.0 .0
    }
    pub const fn column(self) -> i32 {
        self.0 .1
    }
}

impl<T> From<(T, T)> for AxialCoord
where
    T: Into<i32>,
{
    fn from((row, column): (T, T)) -> Self {
        Self::new(row.into(), column.into())
    }
}

impl std::fmt::Display for AxialCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Axial{{ {} {} }}", self.row(), self.column()))
    }
}

pub fn neighbor_coords(coord: AxialCoord) -> impl Iterator<Item = AxialCoord> {
    const ROW_IDS: [i32; 6] = [-1, -1, 0, 1, 1, 0];
    const COLUMN_IDS: [i32; 6] = [0, 1, 1, 0, -1, -1];
    ROW_IDS
        .iter()
        .zip(COLUMN_IDS.iter())
        .map(move |(row, column)| AxialCoord::new(coord.row() + row, coord.column() + column))
}
