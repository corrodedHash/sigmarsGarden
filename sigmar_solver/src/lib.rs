use std::{result, str::FromStr};

mod element;
use element::Element;

mod coord;
use coord::{neighbor_coords, AxialCoord};

mod board;
use board::Board;
use overlay_board::OverlayBoard;
use solving::Combination;

mod overlay_board;

mod solving;

#[test]
fn bla() {
    let mut x = 0..5;
    while let Some(q) = x.next() {
        for n in x.clone() {
            println!("{} {}", q, n);
        }
    }
}

fn format_comb(Combination(one, other): &Combination, board: &OverlayBoard) -> String {
    let cell = board[*one];
    if let Some(other) = other {
        let other_cell = board[*other];
        return format!(
            "Combine {:#?} at {} with {:#?} at {}",
            cell, one, other_cell, other
        );
    } else {
        return format!("{:#?} at {}", cell, one);
    }
}

fn solve_intern(board_str: *const std::os::raw::c_char) -> Result<Vec<Combination>, String> {
    let b = unsafe { std::ffi::CStr::from_ptr(board_str) };
    let q = match b.to_str() {
        Ok(s) => s,
        Err(err) => {
            return Err(format!("UTF8 Error: {:#?}", err));
        }
    };
    let board = match Board::from_str(q) {
        Ok(b) => b,
        Err(err) => {
            return Err(format!("{:#?}", err));
        }
    };
    let oboard = OverlayBoard::from(board.clone());
    for comb in solving::enumerate_combinations(&oboard) {
        println!("{}", format_comb(&comb, &oboard));
    }
    match solving::find_solution(board) {
        Ok(solution) => return Ok(solution),
        Err(err) => return Err("Search field exhausted. No solution found.".to_owned()),
    };
}

#[no_mangle]
pub extern "C" fn solve(board_str: *const std::os::raw::c_char) -> *const std::os::raw::c_char {
    let result = match solve_intern(board_str) {
        Ok(solution) => {
            let json_solution = solving::solution_to_json(&solution);
            serde_json::to_string(&serde_json::json!({ "solution": json_solution }))
        }
        Err(err) => serde_json::to_string(&serde_json::json!({ "error": err })),
    };
    return std::ffi::CString::new(result.unwrap()).unwrap().into_raw();
}
