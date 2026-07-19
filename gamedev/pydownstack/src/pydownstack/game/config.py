from dataclasses import dataclass

from pydownstack.domain.rotation import Rotation
from pydownstack.domain.vector import Vector2D


@dataclass(frozen=True)
class PieceConfig:
    name: str
    coords: list[list[Vector2D]]
    """[rotation][cell_index] -> (dx, dy)"""
    kicks: dict[Rotation, dict[int, list[Vector2D]]]
    """[cw/ccw][to_rot], e.g., [Rotation.CW][2] = offsets for rot 1 -> 2"""
    origin: Vector2D
    """spawn position"""
    width: int


@dataclass(frozen=True)
class GuidelineConfig:
    num_cols: int
    num_rows: int
    num_visible_rows: int
    num_rots: int
    num_previews: int
    pieces: dict[str, PieceConfig]
    scoring: dict[int, int]
    speeds: list[int]
