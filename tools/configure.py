from typing import Any

import cv2
import numpy as np
from sigmarsGarden.config import Configuration
from sigmarsGarden.parse import circle_coords


def configure(img: Any) -> Configuration:

    cv2.namedWindow("configureDisplay")

    # def click_and_crop(event, x, y, flags, param) -> None:
    #     print(event, x, y, flags, param)
    # cv2.setMouseCallback("configureDisplay", click_and_crop)

    cv2.imshow("configureDisplay", img)
    result = Configuration()
    result.down_distance = 114
    result.right_distance = 66
    result.start_coord = (1371, 400)
    result.radius = 28
    circle_color = [0, 0, 0]
    while True:
        keycode = cv2.waitKey(0)
        print(keycode)
        left = 81
        up = 82
        down = 84
        right = 83

        left = 104
        up = 116
        down = 110
        right = 115

        esc = 27

        start_coord = list(result.start_coord)
        if keycode == left:
            start_coord[0] -= 1
        elif keycode == right:
            start_coord[0] += 1
        elif keycode == up:
            start_coord[1] -= 1
        elif keycode == down:
            start_coord[1] += 1
        elif keycode == esc:
            break
        result.start_coord = (start_coord[0], start_coord[1])
        new_img = np.copy(img)
        for coord in circle_coords(result):
            new_img = cv2.circle(new_img, coord, result.radius, circle_color)

        cv2.imshow("configureDisplay", new_img)
        print(start_coord)
    return result


def main() -> None:
    x = cv2.imread("testboards/1.jpg")

    print(configure(x))


if __name__ == "__main__":
    main()
