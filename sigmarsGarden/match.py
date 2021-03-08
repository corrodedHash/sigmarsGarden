from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import cv2
from .catalogue import Catalogue
from .config import R1440P_CONFIG
from .parse import Element, getSquares
from .screenshot import get_screen
import numpy as np


def match_template(template: np.ndarray, search_image: np.ndarray) -> float:
    """Return matchiness for the template on the search image"""
    res = cv2.matchTemplate(search_image, template, cv2.TM_CCOEFF_NORMED)
    min_val, max_val, min_loc, max_loc = cv2.minMaxLoc(res)
    assert isinstance(max_val, (int, float))
    return float(max_val)


def match_squares(
    c: Catalogue, squares: List[Any], debug: bool = False
) -> List[Tuple[Optional[Element], bool]]:
    result = []
    debug_output = []
    for square in squares:
        new_square = square[2:-2, 2:-2]
        a = list(map(lambda x: (x, match_template(x[2], new_square)), c.c))
        a = sorted(a, key=lambda x: x[1])
        a = a[::-1]
        result.append((a[0][0][0], a[0][0][1]))
        if debug:
            debug_output.append(a[:3])
    if debug:
        lines = []
        for orig, line in zip(squares, debug_output):
            lines.append(
                np.concatenate(
                    (orig, *[template[0][2] for template in line], np.zeros_like(orig)),
                    axis=1,
                )
            )
        columns = []
        for column_id in range(7):
            column_count = 91 // 7
            columns.append(
                np.concatenate(
                    tuple(
                        lines[column_id * column_count : (column_id + 1) * column_count]
                    ),
                    axis=0,
                )
            )
        cv2.imshow("output", np.concatenate(tuple(columns), axis=1))
        cv2.waitKey(100)
        input()

    return result