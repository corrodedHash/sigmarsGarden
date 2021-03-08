import json
import math
from typing import Tuple
import cv2
from sigmarsGarden.config import Configuration, R1440P_CONFIG
from sigmarsGarden.acting import coord_to_graphic, origin_from_conf




def main() -> None:
    origin = origin_from_conf(R1440P_CONFIG)
    solution_str = "[[[-5,4],[5,-4]],[[-4,4],[4,-4]],[[1,-5],[-3,4]],[[0,-4],[-1,5]],[[-4,3],[-1,-3]],[[-3,3],[-2,-2]],[[-3,-1],[0,4]],[[4,1],[1,3]],[[-1,-2],[4,-3]],[[0,-3],[-1,4]],[[0,-2],[4,0]],[[4,-1],[1,-4]],[[3,-3],[2,-4]],[[-1,-1],[2,2]],[[4,-2],[1,2]],[[0,3],[3,-1]],[[-2,0],[0,2]],[[-3,0],[3,1]],[[3,0],[-2,1]],[[2,0],[-1,2]],[[-4,-1],[2,-1]],[[-3,1],[3,-4]],[[-2,2],[2,-2]],[[-4,0],[-2,4]],[[-4,2],[2,-3]],[[0,0],null],[[-4,1],[1,1]]]"
    solution = json.loads(solution_str)

    board = cv2.imread("testboards/3.jpg")
    board = cv2.circle(board, origin, R1440P_CONFIG.radius, [255, 255, 255], 7)

    cv2.namedWindow("SolutionDisplay")
    cv2.imshow("SolutionDisplay", board)
    cv2.waitKey(100)
    input()
    for index, [first, second] in enumerate(solution):
        first_coord = coord_to_graphic(origin, first, R1440P_CONFIG)
        board = cv2.circle(
            board, first_coord, R1440P_CONFIG.radius, [0, 0, 0], thickness=4
        )
        if second is not None:
            second_coord = coord_to_graphic(origin, second, R1440P_CONFIG)
            board = cv2.circle(
                board, second_coord, R1440P_CONFIG.radius, [0, 0, 0], thickness=4
            )
        cv2.imshow("SolutionDisplay", board)
        cv2.waitKey(100)
        print(
            f"{index+1} / {len(solution)}", [first_coord, second_coord], [first, second]
        )
        input()


if __name__ == "__main__":
    main()
