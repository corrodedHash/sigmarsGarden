use crate::{
    board::{Board, BOARD_RADIUS, COL_ID_START, ROW_LENGTHS},
    element::Element,
};
use crate::{coord::AxialCoord, solving::Combination};
use std::{
    convert::{TryFrom, TryInto},
    ops::Index,
};

fn to_index(coord: AxialCoord) -> Option<u16> {
    let column_index = u16::try_from(coord.row() + BOARD_RADIUS).ok()?;
    let full_rows_cell_count =
        u16::try_from(ROW_LENGTHS.iter().take(column_index as usize).sum::<i32>())
            .expect("Lengths should be positive");
    let last_row_cell_count = coord.column() - COL_ID_START.get(column_index as usize)?;
    let last_row_cell_count: u16 = u16::try_from(last_row_cell_count).ok()?;

    return Some(full_rows_cell_count + last_row_cell_count);
}
fn to_index_flag(coord: AxialCoord) -> u128 {
    to_index(coord).map(|x| 1 << x).unwrap_or_default()
}

fn index_to_coord(index: u16) -> Option<AxialCoord> {
    if index > 90 {
        return None;
    }
    let mut full_row_count = 0_u16;
    for (row_index, length) in (0_i32..).zip(
        ROW_LENGTHS
            .iter()
            .map(|x| u16::try_from(*x).expect("Should fit")),
    ) {
        if index
            >= full_row_count
                .checked_add(length)
                .expect("Should not overflow")
        {
            full_row_count += length
        } else {
            return Some(AxialCoord::new(
                row_index - BOARD_RADIUS,
                i32::from(index - full_row_count)
                    + COL_ID_START[usize::try_from(row_index).expect("Row index way too high")],
            ));
        }
    }
    return None;
}
fn index_flag_to_coord(flag: u128) -> Option<AxialCoord> {
    index_to_coord(u16::try_from(flag.trailing_zeros()).expect("Should never happen"))
}

#[test]
fn indexing() {
    for (should, is) in
        (0..).zip(crate::board::coord_iterator().map(|x| to_index(x.into()).unwrap()))
    {
        assert_eq!(should, is);
    }
}

#[test]
fn reverse_indexing() {
    for (is, should) in (0..).zip(crate::board::coord_iterator()) {
        assert_eq!(AxialCoord::from(should), index_to_coord(is).unwrap());
    }
}

pub struct OverlayBoard {
    board: Board,
    overlays: Vec<u128>,
    metal: Vec<(Element, u128)>,
    salt: [u128; 4],
}

impl OverlayBoard {
    fn overlay_sum(&self) -> u128 {
        self.overlays.iter().fold(0_u128, |x, y| x | *y)
    }
    fn push_constraint(&mut self, constraint: u128) {
        assert!(constraint & self.overlay_sum() == 0);
        self.overlays.push(constraint)
    }
    pub fn push_combination(&mut self, comb: Combination) -> u128 {
        let constraint = comb.1.map_or_else(
            || to_index_flag(comb.0),
            |x| to_index_flag(comb.0) | to_index_flag(x),
        );
        self.push_constraint(constraint);
        self.overlay_sum()
    }
    pub fn overlay_count(&self) -> usize {
        self.overlays.len()
    }
    pub fn pop(&mut self) -> Option<u128> {
        self.overlays.pop()
    }
    pub fn iterate_tiles(&self) -> Vec<(&AxialCoord, &Element)> {
        self.board
            .iterate_tiles()
            .map(|(c, e)| {
                if to_index_flag(*c) & self.overlay_sum() == 0 {
                    (c, e)
                } else {
                    (c, &Element::EMPTY)
                }
            })
            .collect()
    }
    pub fn won(&self) -> bool {
        self.iterate_tiles()
            .iter()
            .all(|(_, e)| **e == Element::EMPTY)
    }
    pub fn get_next_metal(&self) -> Element {
        let constsum = self.overlay_sum();
        self.metal
            .iter()
            .find_map(|(e, i)| if i & constsum == 0 { Some(*e) } else { None })
            .unwrap_or(Element::EMPTY)
    }
    pub fn get_salted(&self) -> Vec<Element> {
        self.overlays
            .iter()
            .flat_map(|x| {
                self.salt.iter().filter_map(move |s| {
                    if *s & *x == 0 {
                        None
                    } else {
                        Some(self.board[index_flag_to_coord(*x ^ *s).unwrap()])
                    }
                })
            })
            .to_owned()
            .collect()
    }
}

impl Index<AxialCoord> for OverlayBoard {
    type Output = Element;

    fn index(&self, index: AxialCoord) -> &Self::Output {
        if to_index_flag(index) & self.overlay_sum() != 0 {
            return &Element::EMPTY;
        }
        return self.board.index(index);
    }
}

impl From<Board> for OverlayBoard {
    fn from(b: Board) -> Self {
        let mut metals = b
            .iterate_tiles()
            .filter(|(_, e)| e.is_metal())
            .collect::<Vec<_>>();
        metals.sort_by(|(_, e1), (_, e2)| e1.cmp_metal(**e2));
        let metal = metals
            .into_iter()
            .map(|(c, e)| (*e, to_index_flag(*c)))
            .collect();
        let salt = b
            .iterate_tiles()
            .filter_map(|(c, e)| {
                if *e == Element::SALT {
                    Some(to_index_flag(*c))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Self {
            board: b,
            overlays: Vec::default(),
            metal,
            salt: salt.try_into().expect("More than 4 salts"),
        }
    }
}
