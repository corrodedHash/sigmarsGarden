from pathlib import Path
from typing import Any, List, Optional, Tuple

import cv2

from .parse import Element

ELEMENT_STRING_MAP = {
    "f": Element.FIRE,
    "w": Element.WATER,
    "a": Element.AIR,
    "p": Element.PLANT,
    "salt": Element.SALT,
    "quick": Element.QUICKSILVER,
    "lead": Element.LEAD,
    "tin": Element.TIN,
    "iron": Element.IRON,
    "copper": Element.COPPER,
    "silver": Element.SILVER,
    "gold": Element.GOLD,
    "vitae": Element.VITAE,
    "mort": Element.MORT,
}

class Catalogue:
    c: List[Tuple[Optional[Element], bool, Any]]

    def __init__(self, catalogue_path: Path):
        self.c = []
        for f in catalogue_path.iterdir():
            [name, id] = f.stem.split("-")
            alive = name[0].isupper()
            img = cv2.imread(str(f))
            if name == "e":
                self.c.append((None, False, img))
            elif name.lower() in ELEMENT_STRING_MAP:
                self.c.append((ELEMENT_STRING_MAP[name.lower()], alive, img))
            else:
                raise RuntimeError(f"Unknown name {name} of file {f}")
