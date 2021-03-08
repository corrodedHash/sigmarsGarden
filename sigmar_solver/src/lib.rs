use std::str::FromStr;

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

fn format_comb(Combination((one, other)): &Combination, board: &OverlayBoard) -> String {
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

#[no_mangle]
pub extern "C" fn solve(board_str: *const std::os::raw::c_char) -> *const std::os::raw::c_char {
    let b = unsafe { std::ffi::CStr::from_ptr(board_str) };
    let q = b.to_str().unwrap();
    let board = Board::from_str(q).expect("Could not parse board");
    let oboard = OverlayBoard::from(board.clone());
    for comb in solving::enumerate_combinations(&oboard) {
        println!("{}", format_comb(&comb, &oboard));
    }
    solving::find_solution(board);

    let result = std::ffi::CString::new("Works").unwrap();
    return result.into_raw();
}
