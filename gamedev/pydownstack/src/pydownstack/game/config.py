from dataclasses import dataclass
from pathlib import Path
from typing import Any, Self, cast

import yaml

from pydownstack.domain.mino import Mino
from pydownstack.domain.rotation import Rotation
from pydownstack.domain.vector import Vector2D


@dataclass(frozen=True)
class PieceConfig:
    name: str
    coords: list[list[Vector2D]]
    """[rotation][cell_index]"""
    kicks: dict[Rotation, dict[int, list[Vector2D]]]
    """[cw/ccw][to_idx], e.g., [Rotation.CW][1] = offsets for rot 0->R"""
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
    pieces: dict[Mino, PieceConfig]

    _SRS_IDX: list[tuple[str, int]] = [("0", 0), ("R", 1), ("2", 2), ("L", 3)]
    _KEY_TO_ROT: dict[str, Rotation] = {"cw": Rotation.CW, "ccw": Rotation.CCW}

    @classmethod
    def load(cls, path: Path) -> Self:
        with path.open() as f:
            raw = yaml.safe_load(f)
        return cls._validate(raw)

    @classmethod
    def _validate(cls, raw: Any) -> Self:
        if not isinstance(raw, dict):
            raise ConfigError("Guideline config must be a mapping")

        raw = cast(dict[str, Any], raw)

        raw_pieces = _require_key(d=raw, key="pieces", expected=dict[str, Any])
        pieces: dict[Mino, PieceConfig] = {}
        for name, piece in raw_pieces.items():
            try:
                mino = Mino[name.upper()]
            except KeyError:
                raise ConfigError(f"Unknown piece: {name}")

            if mino in (Mino.EMPTY, Mino.GARBAGE):
                raise ConfigError(f"Piece name collides with sentinel: {name}")

            coords_raw = _require_key(
                d=piece, key="coords", expected=dict[str, list[list[int]]]
            )
            coords: list[list[Vector2D]] = []
            for state_key, _ in sorted(cls._SRS_IDX, key=lambda p: p[1]):
                cells_raw = _require_key(
                    d=coords_raw, key=state_key, expected=list[list[int]]
                )
                coords.append([Vector2D(x=c[0], y=c[1]) for c in cells_raw])

            kicks_raw = _require_key(
                d=piece,
                key="kicks",
                expected=dict[str, dict[str, list[list[int]]]],
                allow_empty=(mino == Mino.O),
            )
            kicks: dict[Rotation, dict[int, list[Vector2D]]] = {}
            for dir_key, rot in cls._KEY_TO_ROT.items():
                kicks[rot] = {}
                dir_kicks_raw = _require_key(
                    d=kicks_raw,
                    key=dir_key,
                    expected=dict[str, list[list[int]]],
                    path=f"kicks.{dir_key}",
                )
                for state_key, to_idx in cls._SRS_IDX:
                    offsets_raw = _require_key(
                        d=dir_kicks_raw,
                        key=state_key,
                        expected=list[list[int]],
                        path=f"kicks.{dir_key}.{state_key}",
                    )
                    kicks[rot][to_idx] = [Vector2D(x=o[0], y=o[1]) for o in offsets_raw]

            origin_raw = _require_key(d=piece, key="origin", expected=list[int])
            if len(origin_raw) != 2:
                raise ConfigError(f"{name}.origin must be [col, row]")
            origin = Vector2D(x=origin_raw[0], y=origin_raw[1])

            width = _require_key(d=piece, key="width", expected=int)

            pieces[mino] = PieceConfig(
                name=name, coords=coords, kicks=kicks, origin=origin, width=width
            )

        return cls(
            num_cols=_require_key(d=raw, key="num_cols", expected=int),
            num_rows=_require_key(d=raw, key="num_rows", expected=int),
            num_visible_rows=_require_key(d=raw, key="num_visible_rows", expected=int),
            num_rots=_require_key(d=raw, key="num_rots", expected=int),
            num_previews=_require_key(d=raw, key="num_previews", expected=int),
            pieces=pieces,
        )


class ConfigError(ValueError):
    """Malformed guideline configuration."""


def _require_key[T](
    d: dict[str, Any],
    key: str,
    expected: type[T],
    path: str = "",
    allow_empty: bool = False,
) -> T:
    full = f"{path}.{key}" if path else key
    if key not in d:
        raise ConfigError(f"Missing required key: {full}")

    val = d[key]
    if not isinstance(val, expected):
        raise ConfigError(
            f"{full}: expected {expected.__name__}, got {type(val).__name__}"
        )
    if isinstance(val, (dict, list)) and not val and not allow_empty:
        raise ConfigError(f"{full}: must not be empty")

    return val
