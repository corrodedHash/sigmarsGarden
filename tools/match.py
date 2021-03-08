from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import cv2
from sigmarsGarden.catalogue import Catalogue
from sigmarsGarden.config import R1440P_CONFIG
from sigmarsGarden.parse import getSquares
from sigmarsGarden.screenshot import get_screen
from sigmarsGarden.match import match_squares
import numpy as np


def main() -> None:
    c = Catalogue(Path("/home/lukas/templates"))

    x = cv2.imread("testboards/3.jpg")
    # x = get_screen()

    squares = getSquares(x, R1440P_CONFIG)
    match_squares(c, squares, debug=True)


if __name__ == "__main__":
    main()
