from typing import Tuple
import math
from .config import Configuration


def origin_from_conf(conf: Configuration) -> Tuple[int, int]:
    return (
        conf.start_coord[0] + math.floor(2.5 * conf.right_distance),
        conf.start_coord[1] + math.floor(2.5 * conf.down_distance),
    )


def coord_to_graphic(
    origin: Tuple[int, int], coord: Tuple[int, int], conf: Configuration
) -> Tuple[int, int]:
    column = coord[1]
    row = coord[0]
    return (
        origin[0] + math.floor((column + (row / 2)) * conf.right_distance),
        origin[1] + math.floor(row * conf.down_distance / 2),
    )