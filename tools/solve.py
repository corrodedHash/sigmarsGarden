import ctypes
import json
from ctypes import cdll
from pathlib import Path
from typing import Any, List, Optional, Tuple
import time

import pyautogui

from sigmarsGarden.catalogue import Catalogue
from sigmarsGarden.config import Configuration, R1440P_CONFIG
from sigmarsGarden.match import match_squares
from sigmarsGarden.parse import Element, getSquares
from sigmarsGarden.screenshot import get_screen
from sigmarsGarden.acting import coord_to_graphic, origin_from_conf
import cv2

pyautogui.FAILSAFE = False
pyautogui.PAUSE = 0


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
    lib = cdll.LoadLibrary("sigmar_solver/target/release/sigmar_solver.dll")
    lib.solve.restype = ctypes.c_char_p
    lib.solve.argtypes = [ctypes.c_char_p]

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

    click_speed = 0.01
    mouse_up_speed = 0.01
    move_speed = 0.02

    def click_coord(coord) -> None:
        real_coord = coord_to_graphic(origin, coord, conf)
        pyautogui.moveTo(real_coord[0], real_coord[1])
        time.sleep(move_speed)
        pyautogui.mouseDown()
        time.sleep(click_speed)
        pyautogui.mouseUp()
        time.sleep(mouse_up_speed)

    for [step_one, step_two] in solution:
        click_coord(step_one)
        if step_two is not None:
            click_coord(step_two)


def loop(c: Catalogue, conf: Configuration) -> None:
    print("Taking Screenshot")
    x = get_screen()
    # x = cv2.imread("testboards/3.jpg")
    # cv2.imshow("preview",x)
    # cv2.waitKey(100)
    # input()
    json_board = parse(c, x, conf)
    # print(json_board)
    # input()
    solution = solve(json_board)
    if solution is None:
        return
    act(solution, conf)


def main() -> None:

    # c = Catalogue(Path("/home/lukas/templates"))
    c = Catalogue(Path("E:/templates"))

    print("Sleeping")
    time.sleep(3)

    for i in range(100):
        loop(c, R1440P_CONFIG)
        time.sleep(2)

        pyautogui.moveTo(1200, 1070)
        time.sleep(0.2)
        pyautogui.mouseDown()
        time.sleep(1)
        pyautogui.mouseUp()
        time.sleep(4)


if __name__ == "__main__":
    main()
