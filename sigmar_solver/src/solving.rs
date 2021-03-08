use std::{collections::HashSet, fmt::Display};

use crate::{
    board::Board,
    coord::{neighbor_coords, AxialCoord},
    element::Element,
    overlay_board::OverlayBoard,
};

fn is_alive(board: &OverlayBoard, coord: AxialCoord) -> bool {
    let mut empty_count = 0;
    let cell_element = board[coord];
    if cell_element == Element::EMPTY {
        return false;
    }
    if cell_element.is_metal() && cell_element != board.get_next_metal() {
        return false;
    }
    for neighbor in neighbor_coords(coord).chain(neighbor_coords(coord).take(2)) {
        if board[neighbor] == Element::EMPTY {
            empty_count += 1;
            if empty_count == 3 {
                return true;
            }
        } else {
            empty_count = 0;
        }
    }
    return false;
}

fn can_salt(board: &OverlayBoard, element: Element) -> bool {
    let salts = board.get_salted();
    return match salts.len() {
        0 => true,
        1 => true,
        2 => salts.iter().any(|x| *x == element),
        3 => {
            let my_element_count = salts.iter().filter(|x| **x == element).count();
            my_element_count == 1 || my_element_count == 3
        }
        _ => false,
    };
}

fn salt_check(board: &OverlayBoard, cell: Element, other_cell: Element) -> bool {
    if cell == Element::SALT && !can_salt(&board, other_cell) {
        return false;
    }
    if other_cell == Element::SALT && !can_salt(&board, cell) {
        return false;
    }
    return true;
}

fn enumerate_alive(board: &OverlayBoard) -> Vec<AxialCoord> {
    let tiles = board.iterate_tiles();
    tiles
        .iter()
        .filter_map(|(c, e)| {
            if **e != Element::EMPTY && is_alive(board, **c) {
                Some(**c)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Clone, Debug, Copy)]
pub struct Combination(pub AxialCoord, pub Option<AxialCoord>);

impl Display for Combination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = self.1 {
            f.write_fmt(format_args!("Delete {} & {}", self.0, x))
        } else {
            f.write_fmt(format_args!("Delete {}", self.0))
        }
    }
}

pub fn enumerate_combinations(board: &OverlayBoard) -> Vec<Combination> {
    let mut result = vec![];
    let alive_vec = enumerate_alive(board);
    let mut alive = alive_vec.into_iter();
    while let Some(x) = alive.next() {
        let next_ones = alive.clone();
        let cell = board[x];
        if cell == Element::GOLD {
            result.push((x, None));
            continue;
        }
        for n in next_ones {
            let other_cell = board[n];
            if cell.can_match(other_cell) && salt_check(&board, cell, other_cell) {
                result.push((x, Some(n)));
            }
        }
    }
    return result.into_iter().map(|(x, y)| Combination(x, y)).collect();
}

pub fn solution_from_stack(option_stack: &[Vec<Combination>]) -> Vec<Combination> {
    return option_stack
        .into_iter()
        .map(|x| *x.last().unwrap())
        .collect();
}

pub fn solution_to_json(solution: &[Combination]) -> serde_json::Value {
    let mapped = &solution
        .iter()
        .map(|Combination(d, o): &Combination| {
            ((d.row(), d.column()), o.map(|x| (x.row(), x.column())))
        })
        .collect::<Vec<((i32, i32), Option<(i32, i32)>)>>();
    serde_json::json!(mapped)
}

pub fn find_solution(board: Board) -> Result<Vec<Combination>, ()> {
    let mut seen_hashes = HashSet::<u128>::default();
    let mut overlay_board = OverlayBoard::from(board);
    let mut option_stack: Vec<Vec<Combination>> = Vec::default();
    while !overlay_board.won() {
        assert_eq!(overlay_board.overlay_count(), option_stack.len());

        option_stack.push(enumerate_combinations(&overlay_board));
        let mut pushed_successful = false;
        while let Some(comb) = option_stack.last().unwrap().last() {
            if !seen_hashes.insert(overlay_board.push_combination(*comb)) {
                overlay_board.pop();
                option_stack.last_mut().unwrap().pop();
            } else {
                pushed_successful = true;
                break;
            }
        }
        if !pushed_successful {
            option_stack.pop();

            while let Some(last_options) = option_stack.last_mut() {
                last_options.pop();
                overlay_board.pop();
                if let Some(next_comb) = last_options.last() {
                    overlay_board.push_combination(*next_comb);
                    assert_eq!(overlay_board.overlay_count(), option_stack.len());

                    break;
                } else {
                    option_stack.pop();
                    assert_eq!(overlay_board.overlay_count(), option_stack.len());
                    continue;
                }
            }
            if option_stack.is_empty() {
                return Err(());
            }
        }
    }
    return Ok(option_stack
        .into_iter()
        .map(|x| *x.last().unwrap())
        .collect());
}
