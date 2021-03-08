import ctypes
import json
from ctypes import cdll
from pathlib import Path
from typing import Any, List, Optional, Tuple

import pyautogui

from sigmarsGarden.catalogue import Catalogue
from sigmarsGarden.config import Configuration, R1440P_CONFIG
from sigmarsGarden.match import match_squares
from sigmarsGarden.parse import Element, getSquares
from sigmarsGarden.screenshot import get_screen
from sigmarsGarden.acting import coord_to_graphic, origin_from_conf
import cv2

lib = cdll.LoadLibrary("sigmar_solver/target/release/libsigmar_solver.so")
lib.solve.restype = ctypes.c_char_p
lib.solve.argtypes = [ctypes.c_char_p]


def parse(c: Catalogue, img: Any, conf: Configuration) -> str:
    squares = getSquares(img, conf)
    tiles = match_squares(c, squares)
    mapped_tiles = list(
        map(lambda x: x[0].name if x[0] is not None else "EMPTY", tiles)
    )

    return json.dumps(mapped_tiles)


COORD = Tuple[int, int]


def solve(
    board: str,
) -> Optional[List[Tuple[COORD, Optional[COORD]]]]:
    result = lib.solve(ctypes.c_char_p(board.encode("utf-8")))
    decoded_result = json.loads(result.decode("utf-8"))

    print(decoded_result)
    if "error" in decoded_result:
        print(decoded_result["error"])
        return None
    else:
        return decoded_result["solution"]


def act(solution: List[Tuple[COORD, Optional[COORD]]], conf: Configuration) -> None:
    origin = origin_from_conf(conf)
    for [step_one, step_two] in solution:
        real_coord_one = coord_to_graphic(origin, step_one, conf)
        pyautogui.moveTo(real_coord_one[0], real_coord_one[1])
        pyautogui.click(
            real_coord_one[0],
            real_coord_one[1],
            duration=0.5,
            tween=pyautogui.easeInQuad,
        )
        if step_two is not None:
            real_coord_two = coord_to_graphic(origin, step_two, conf)
            pyautogui.moveTo(real_coord_two[0], real_coord_two[1])
            pyautogui.click(
                real_coord_two[0],
                real_coord_two[1],
                duration=0.5,
                tween=pyautogui.easeInQuad,
            )   


def main() -> None:

    c = Catalogue(Path("/home/lukas/templates"))
    conf = R1440P_CONFIG
    # x = get_screen()
    x = cv2.imread("testboards/3.jpg")

    json_board = parse(c, x, conf)
    solution = solve(json_board)
    if solution is None:
        return
    act(solution, conf)


if __name__ == "__main__":
    main()
