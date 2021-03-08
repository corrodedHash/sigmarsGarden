import enum
import math
import typing
from enum import Enum
from typing import Any, Dict, Iterator, List, Optional, Tuple

import cv2
import numpy as np

from .config import Configuration


def circle_coords(conf: Configuration) -> Iterator[Tuple[int, int]]:
    def even_line(row: int) -> Iterator[Tuple[int, int]]:
        extra_cells = min(abs(5 - row), row)
        for i in range(6 + extra_cells * 2):
            new_coord = (
                conf.start_coord[0] + conf.right_distance * (i - extra_cells),
                conf.start_coord[1] + conf.down_distance * row,
            )
            yield new_coord

    def odd_line(row: int) -> Iterator[Tuple[int, int]]:
        extra_cells = min(abs(4 - row), row)
        for i in range(7 + extra_cells * 2):
            new_coord = (
                conf.start_coord[0]
                + conf.right_distance * (i - extra_cells)
                - conf.right_distance // 2,
                conf.start_coord[1]
                + conf.down_distance * row
                + conf.down_distance // 2,
            )
            yield new_coord

    for row in range(11):
        if row % 2 == 0:
            yield from even_line(row // 2)
        else:
            yield from odd_line(row // 2)


def getSquares(img: Any, conf: Configuration) -> List[Any]:
    square_halfside = conf.radius / math.sqrt(2)
    squares = []
    for coord in circle_coords(conf):
        horizontal_slice = slice(
            math.floor(coord[0] - square_halfside),
            math.ceil(coord[0] + square_halfside),
        )
        vertical_slice = slice(
            math.floor(coord[1] - square_halfside),
            math.ceil(coord[1] + square_halfside),
        )
        squares.append(img[vertical_slice, horizontal_slice])
    return squares


@enum.unique
class Element(Enum):
    VITAE = enum.auto()
    MORT = enum.auto()

    AIR = enum.auto()
    FIRE = enum.auto()
    WATER = enum.auto()
    PLANT = enum.auto()
    SALT = enum.auto()

    QUICKSILVER = enum.auto()
    LEAD = enum.auto()
    TIN = enum.auto()
    IRON = enum.auto()
    COPPER = enum.auto()
    SILVER = enum.auto()
    GOLD = enum.auto()


class Board:
    _internal_storage: Dict[Tuple[int, int], Element]

    def __init__(self) -> None:
        pass

    def get_cell(self, row: int, column: int) -> Optional[Element]:
        return self._internal_storage.get((row, column))


def parse_board(image: Any) -> Board:
    pass
