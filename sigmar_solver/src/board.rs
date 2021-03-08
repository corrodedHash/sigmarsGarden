use std::{collections::HashMap, fmt::Debug, ops::Index, str::FromStr};

use crate::{coord::AxialCoord, element::Element};

pub const ROW_LENGTHS: [i32; 11] = [6, 7, 8, 9, 10, 11, 10, 9, 8, 7, 6];
pub const COL_ID_START: [i32; 11] = [0, -1, -2, -3, -4, -5, -5, -5, -5, -5, -5];
pub const BOARD_RADIUS: i32 = 5;

pub fn coord_iterator() -> impl Iterator<Item = (i32, i32)> {
    let row_id = -BOARD_RADIUS..=BOARD_RADIUS;
    let coord_iterator = ROW_LENGTHS
        .iter()
        .zip(COL_ID_START.iter())
        .zip(row_id)
        .flat_map(|((length, col_id), row_id)| (0..*length).map(move |x| (row_id, col_id + x)));
    return coord_iterator;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Board {
    internal_storage: HashMap<AxialCoord, Element>,
}
impl Board {
    pub fn iterate_tiles(&self) -> impl Iterator<Item = (&AxialCoord, &Element)> {
        self.internal_storage.iter()
    }
    pub fn check(&self) -> Result<(), Vec<(Element, i32)>> {
        let mut should: HashMap<Element, i32> = vec![
            (Element::SALT, 4),
            (Element::AIR, 8),
            (Element::FIRE, 8),
            (Element::WATER, 8),
            (Element::PLANT, 8),
            (Element::QUICKSILVER, 5),
            (Element::LEAD, 1),
            (Element::TIN, 1),
            (Element::IRON, 1),
            (Element::COPPER, 1),
            (Element::SILVER, 1),
            (Element::GOLD, 1),
            (Element::VITAE, 4),
            (Element::MORT, 4),
        ]
        .into_iter()
        .collect();
        for (_, e) in self.iterate_tiles() {
            if *e == Element::EMPTY {
                continue;
            }
            *should.get_mut(e).unwrap() -= 1;
        }
        let rest = should
            .into_iter()
            .filter(|(_, count)| *count != 0)
            .collect::<Vec<_>>();
        if !rest.is_empty() {
            return Err(rest);
        }
        return Ok(());
    }
}

#[derive(Debug)]
pub enum ParsingError {
    Parsing(serde_json::Error),
    Logic(Vec<(Element, i32)>),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::Parsing(s) => std::fmt::Display::fmt(&s, f),
            ParsingError::Logic(vec) => {
                f.write_fmt(format_args!("Element count mismatch: {:#?}", vec))
            }
        }
    }
}

impl From<serde_json::Error> for ParsingError {
    fn from(x: serde_json::Error) -> Self {
        Self::Parsing(x)
    }
}
impl From<Vec<(Element, i32)>> for ParsingError {
    fn from(x: Vec<(Element, i32)>) -> Self {
        Self::Logic(x)
    }
}

impl FromStr for Board {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Element> = serde_json::from_str(s)?;
        let tile_map = tiles
            .iter()
            .zip(coord_iterator())
            .map(|(t, c)| (AxialCoord::from(c), *t))
            .collect();
        let board = Self {
            internal_storage: tile_map,
        };
        board.check()?;
        return Ok(board);
    }
}

impl Index<AxialCoord> for Board {
    type Output = Element;

    fn index(&self, index: AxialCoord) -> &Self::Output {
        self.internal_storage.get(&index).unwrap_or(&Element::EMPTY)
    }
}
