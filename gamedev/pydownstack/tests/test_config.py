import copy
from pathlib import Path
from typing import Any

import pytest
import yaml

from pydownstack.domain.mino import Mino
from pydownstack.game.config import ConfigError, GuidelineConfig

# ── fixtures ─────────────────────────────────────────────────────────

_GUIDELINE = Path("src/pydownstack/resources/guideline.yml")


@pytest.fixture(scope="session")
def guideline_raw() -> dict[str, Any]:
    with _GUIDELINE.open() as f:
        return yaml.safe_load(f)


@pytest.fixture
def j_piece(guideline_raw: dict[str, Any]) -> dict[str, Any]:
    return guideline_raw["pieces"]["J"]


@pytest.fixture
def o_piece(guideline_raw: dict[str, Any]) -> dict[str, Any]:
    raw = guideline_raw["pieces"]["O"]
    return {**raw, "kicks": {}}  # O has no kicks


@pytest.fixture
def config(j_piece: dict[str, Any]) -> dict[str, Any]:
    return {
        "num_cols": 10,
        "num_rows": 25,
        "num_visible_rows": 20,
        "num_rots": 4,
        "num_previews": 5,
        "pieces": {"J": copy.deepcopy(j_piece)},
    }


# ── valid configs ────────────────────────────────────────────────────


def test_minimal_valid(config: dict[str, Any]) -> None:
    cfg = GuidelineConfig._validate(config)
    assert cfg.num_cols == 10
    assert cfg.num_rows == 25
    assert Mino.J in cfg.pieces
    assert cfg.pieces[Mino.J].width == 3


def test_all_seven_pieces(
    guideline_raw: dict[str, Any], o_piece: dict[str, Any]
) -> None:
    raw = guideline_raw["pieces"]
    cfg = GuidelineConfig._validate(
        {
            "num_cols": 10,
            "num_rows": 25,
            "num_visible_rows": 20,
            "num_rots": 4,
            "num_previews": 5,
            "pieces": {
                "J": raw["J"],
                "L": raw["L"],
                "S": raw["S"],
                "Z": raw["Z"],
                "T": raw["T"],
                "I": raw["I"],
                "O": o_piece,
            },
        }
    )
    assert len(cfg.pieces) == 7
    assert cfg.pieces[Mino.O].kicks == {}


def test_srs_state_mapping(config: dict[str, Any]) -> None:
    cfg = GuidelineConfig._validate(config)
    pc = cfg.pieces[Mino.J]
    assert len(pc.coords) == 4


def test_kick_structure(config: dict[str, Any]) -> None:
    cfg = GuidelineConfig._validate(config)
    kicks = cfg.pieces[Mino.J].kicks
    assert len(kicks) == 2  # CW + CCW


def test_load_from_file() -> None:
    if not _GUIDELINE.exists():
        pytest.skip("guideline.yml not found")
    cfg = GuidelineConfig.load(_GUIDELINE)
    assert len(cfg.pieces) == 7


# ── missing keys ─────────────────────────────────────────────────────


def test_missing_top_level_key(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["num_cols"]
    with pytest.raises(ConfigError, match="Missing required key: num_cols"):
        GuidelineConfig._validate(d)


def test_missing_pieces_key() -> None:
    with pytest.raises(ConfigError, match="Missing required key: pieces"):
        GuidelineConfig._validate(
            {
                "num_cols": 10,
                "num_rows": 25,
                "num_visible_rows": 20,
                "num_rots": 4,
                "num_previews": 5,
            }
        )


def test_missing_coords_key(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["coords"]
    with pytest.raises(ConfigError, match="Missing required key: coords"):
        GuidelineConfig._validate(d)


def test_missing_srs_state_in_coords(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["coords"]["R"]
    with pytest.raises(ConfigError, match="Missing required key: coords.R"):
        GuidelineConfig._validate(d)


def test_missing_kicks_key(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["kicks"]
    with pytest.raises(ConfigError, match="Missing required key: kicks"):
        GuidelineConfig._validate(d)


def test_missing_kick_direction(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["kicks"]["cw"]
    with pytest.raises(ConfigError, match="Missing required key: kicks.cw"):
        GuidelineConfig._validate(d)


def test_missing_kick_state(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["kicks"]["cw"]["R"]
    with pytest.raises(ConfigError, match="Missing required key: kicks.cw.R"):
        GuidelineConfig._validate(d)


def test_missing_origin(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["origin"]
    with pytest.raises(ConfigError, match="Missing required key: origin"):
        GuidelineConfig._validate(d)


def test_missing_width(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]["width"]
    with pytest.raises(ConfigError, match="Missing required key: width"):
        GuidelineConfig._validate(d)


# ── wrong types ──────────────────────────────────────────────────────


def test_top_level_wrong_type(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["num_cols"] = "ten"
    with pytest.raises(ConfigError, match="num_cols: expected int"):
        GuidelineConfig._validate(d)


def test_pieces_not_dict(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"] = []
    with pytest.raises(ConfigError, match="pieces: expected dict"):
        GuidelineConfig._validate(d)


def test_coords_not_dict(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["coords"] = [[0, 1]]
    with pytest.raises(ConfigError, match="coords: expected dict"):
        GuidelineConfig._validate(d)


def test_coord_cells_not_list(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["coords"]["0"] = "bad"
    with pytest.raises(ConfigError, match="coords.0: expected list"):
        GuidelineConfig._validate(d)


def test_origin_wrong_length(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["origin"] = [3]
    with pytest.raises(ConfigError, match="origin must be"):
        GuidelineConfig._validate(d)


def test_width_wrong_type(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["width"] = "3"
    with pytest.raises(ConfigError, match="width: expected int"):
        GuidelineConfig._validate(d)


# ── unknown piece ────────────────────────────────────────────────────


def test_unknown_piece(config: dict[str, Any], j_piece: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["X"] = j_piece
    with pytest.raises(ConfigError, match="Unknown piece: X"):
        GuidelineConfig._validate(d)


def test_piece_name_collides_with_empty(
    config: dict[str, Any], j_piece: dict[str, Any]
) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]
    d["pieces"]["EMPTY"] = j_piece
    with pytest.raises(ConfigError, match="collides with sentinel"):
        GuidelineConfig._validate(d)


# ── empty checks ─────────────────────────────────────────────────────


def test_empty_pieces_dict_rejected(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"] = {}
    with pytest.raises(ConfigError, match="pieces: must not be empty"):
        GuidelineConfig._validate(d)


def test_empty_coords_rejected(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["coords"] = {}
    with pytest.raises(ConfigError, match="coords: must not be empty"):
        GuidelineConfig._validate(d)


def test_empty_kicks_allowed_for_o(o_piece: dict[str, Any]) -> None:
    d = {
        "num_cols": 10,
        "num_rows": 25,
        "num_visible_rows": 20,
        "num_rots": 4,
        "num_previews": 5,
        "pieces": {"O": o_piece},
    }
    cfg = GuidelineConfig._validate(d)
    assert cfg.pieces[Mino.O].kicks == {}


def test_empty_kicks_rejected_for_non_o(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["pieces"]["J"]["kicks"] = {}
    with pytest.raises(ConfigError, match="kicks: must not be empty"):
        GuidelineConfig._validate(d)


# ── edge cases ───────────────────────────────────────────────────────


def test_root_not_dict() -> None:
    with pytest.raises(ConfigError, match="must be a mapping"):
        GuidelineConfig._validate([])


def test_case_insensitive_piece_name(
    config: dict[str, Any], j_piece: dict[str, Any]
) -> None:
    d = copy.deepcopy(config)
    del d["pieces"]["J"]
    d["pieces"]["j"] = j_piece
    cfg = GuidelineConfig._validate(d)
    assert Mino.J in cfg.pieces


def test_extra_keys_ignored(config: dict[str, Any]) -> None:
    d = copy.deepcopy(config)
    d["unknown"] = "ignored"
    cfg = GuidelineConfig._validate(d)
    assert cfg.num_cols == 10
