import ctypes
import json
from ctypes import cdll
from pathlib import Path

from sigmarsGarden.catalogue import Catalogue
from sigmarsGarden.config import R1440P_CONFIG
from sigmarsGarden.match import match_squares
from sigmarsGarden.parse import Element, getSquares
from sigmarsGarden.screenshot import get_screen
import cv2

lib = cdll.LoadLibrary("sigmar_solver/target/debug/libsigmar_solver.so")
lib.solve.restype = ctypes.c_char_p
lib.solve.argtypes = [ctypes.c_char_p]


def main() -> None:

    c = Catalogue(Path("/home/lukas/templates"))

    # x = get_screen()
    x = cv2.imread("testboards/3.jpg")

    squares = getSquares(x, R1440P_CONFIG)
    tiles = match_squares(c, squares)
    mapped_tiles = list(
        map(lambda x: x[0].name if x[0] is not None else "EMPTY", tiles)
    )

    sending = json.dumps(mapped_tiles)
    result = lib.solve(ctypes.c_char_p(sending.encode("utf-8")))
    print(result)
    print(result.decode("utf-8"))


if __name__ == "__main__":
    main()
