import math
import time
from pathlib import Path
import tempfile
from typing import Any, Iterator, List, Tuple

import cv2
import numpy as np
import pyautogui

from .parse import Element
from .config import Configuration


def get_screen() -> Any:
    pil_screenshot = pyautogui.screenshot()
    with tempfile.TemporaryDirectory() as tmpdirname:
        path = tmpdirname + "/bla.png"
        pil_screenshot.save(path)
        np_screenshot = cv2.imread(path)

    cropped_screenshot = np_screenshot[0:1440, 1920:]

    return cropped_screenshot
