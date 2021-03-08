from typing import Tuple


class Configuration:
    start_coord: Tuple[int, int]
    right_distance: int
    down_distance: int
    radius: int


R1440P_CONFIG = Configuration()
R1440P_CONFIG.down_distance = 114
R1440P_CONFIG.right_distance = 66
R1440P_CONFIG.start_coord = (1371, 400)
R1440P_CONFIG.radius = 28
