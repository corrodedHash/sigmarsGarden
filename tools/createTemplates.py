from sigmarsGarden.parse import getSquares
from sigmarsGarden.config import Configuration, R1440P_CONFIG
import cv2
from pathlib import Path
from typing import List, Any


def create_templates(path: Path, templates: List[Any]) -> None:
    def save_image(filename: str, img: Any) -> None:
        extension = "png"
        counter = 0
        while (path / f"{filename}-{counter:03}.{extension}").exists():
            counter += 1
        result_path = path / f"{filename}-{counter:03}.{extension}"
        cv2.imwrite(str(result_path), img)
        print(f"Written to {result_path}")

    cv2.namedWindow("Template")
    cv2.imshow("Template", templates[0])
    for temp in templates:
        cv2.imshow("Template", temp)
        cv2.waitKey(100)
        selection = input().lower()
        selection.replace(" ", "_")
        save_image(selection, temp)


def main() -> None:
    x = cv2.imread("testboards/1.jpg")

    squares = getSquares(x, R1440P_CONFIG)
    create_templates(Path("/home/lukas/templates"), squares)


if __name__ == "__main__":
    main()