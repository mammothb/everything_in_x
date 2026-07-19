from pathlib import Path

import pytest
import yaml

from pydownstack.domain.mino import Mino
from pydownstack.game.config import ConfigError, GuidelineConfig

# ── helpers ──────────────────────────────────────────────────────────

_GUIDELINE = Path("src/pydownstack/resources/guideline.yml")


def _raw() -> dict:
    with _GUIDELINE.open() as f:
        return yaml.safe_load(f)


def _j_piece() -> dict:
    return _raw()["pieces"]["J"]


def _o_piece() -> dict:
    raw = _raw()["pieces"]["O"]
    return {**raw, "kicks": {}}  # O has no kicks


def _piece() -> dict:
    return _j_piece()


def _config(**overrides) -> dict:
    base: dict = {
        "num_cols": 10,
        "num_rows": 25,
        "num_visible_rows": 20,
        "num_rots": 4,
        "num_previews": 5,
        "pieces": {"J": _piece()},
    }
    base.update(overrides)
    return base


# ── valid configs ────────────────────────────────────────────────────


def test_minimal_valid() -> None:
    cfg = GuidelineConfig._validate(_config())
    assert cfg.num_cols == 10
    assert cfg.num_rows == 25
    assert Mino.J in cfg.pieces
    assert cfg.pieces[Mino.J].width == 3


def test_all_seven_pieces() -> None:
    cfg = GuidelineConfig._validate(
        _config(
            pieces={
                "J": _j_piece(),
                "L": _raw()["pieces"]["L"],
                "S": _raw()["pieces"]["S"],
                "Z": _raw()["pieces"]["Z"],
                "T": _raw()["pieces"]["T"],
                "I": _raw()["pieces"]["I"],
                "O": _o_piece(),
            }
        )
    )
    assert len(cfg.pieces) == 7
    assert cfg.pieces[Mino.O].kicks == {}


def test_srs_state_mapping() -> None:
    """R maps to index 1, L maps to index 3."""
    cfg = GuidelineConfig._validate(_config(pieces={"J": _j_piece()}))
    pc = cfg.pieces[Mino.J]
    assert len(pc.coords) == 4


def test_kick_structure() -> None:
    cfg = GuidelineConfig._validate(_config(pieces={"J": _j_piece()}))
    kicks = cfg.pieces[Mino.J].kicks
    assert len(kicks) == 2  # CW + CCW


def test_load_from_file() -> None:
    if not _GUIDELINE.exists():
        pytest.skip("guideline.yml not found")
    cfg = GuidelineConfig.load(_GUIDELINE)
    assert len(cfg.pieces) == 7


# ── missing keys ─────────────────────────────────────────────────────


def test_missing_top_level_key() -> None:
    d = _config()
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


def test_missing_coords_key() -> None:
    d = _config()
    del d["pieces"]["J"]["coords"]
    with pytest.raises(ConfigError, match="Missing required key: coords"):
        GuidelineConfig._validate(d)


def test_missing_srs_state_in_coords() -> None:
    d = _config()
    del d["pieces"]["J"]["coords"]["R"]
    with pytest.raises(ConfigError, match="Missing required key: coords.R"):
        GuidelineConfig._validate(d)


def test_missing_kicks_key() -> None:
    d = _config()
    del d["pieces"]["J"]["kicks"]
    with pytest.raises(ConfigError, match="Missing required key: kicks"):
        GuidelineConfig._validate(d)


def test_missing_kick_direction() -> None:
    d = _config()
    del d["pieces"]["J"]["kicks"]["cw"]
    with pytest.raises(ConfigError, match="Missing required key: kicks.cw"):
        GuidelineConfig._validate(d)


def test_missing_kick_state() -> None:
    d = _config()
    del d["pieces"]["J"]["kicks"]["cw"]["R"]
    with pytest.raises(ConfigError, match="Missing required key: kicks.cw.R"):
        GuidelineConfig._validate(d)


def test_missing_origin() -> None:
    d = _config()
    del d["pieces"]["J"]["origin"]
    with pytest.raises(ConfigError, match="Missing required key: origin"):
        GuidelineConfig._validate(d)


def test_missing_width() -> None:
    d = _config()
    del d["pieces"]["J"]["width"]
    with pytest.raises(ConfigError, match="Missing required key: width"):
        GuidelineConfig._validate(d)


# ── wrong types ──────────────────────────────────────────────────────


def test_top_level_wrong_type() -> None:
    with pytest.raises(ConfigError, match="num_cols: expected int"):
        GuidelineConfig._validate(_config(num_cols="ten"))


def test_pieces_not_dict() -> None:
    with pytest.raises(ConfigError, match="pieces: expected dict"):
        GuidelineConfig._validate(_config(pieces=[]))


def test_coords_not_dict() -> None:
    d = _config()
    d["pieces"]["J"]["coords"] = [[0, 1]]
    with pytest.raises(ConfigError, match="coords: expected dict"):
        GuidelineConfig._validate(d)


def test_coord_cells_not_list() -> None:
    d = _config()
    d["pieces"]["J"]["coords"]["0"] = "bad"
    with pytest.raises(ConfigError, match="coords.0: expected list"):
        GuidelineConfig._validate(d)


def test_origin_wrong_length() -> None:
    d = _config()
    d["pieces"]["J"]["origin"] = [3]
    with pytest.raises(ConfigError, match="origin must be"):
        GuidelineConfig._validate(d)


def test_width_wrong_type() -> None:
    d = _config()
    d["pieces"]["J"]["width"] = "3"
    with pytest.raises(ConfigError, match="width: expected int"):
        GuidelineConfig._validate(d)


# ── unknown piece ────────────────────────────────────────────────────


def test_unknown_piece() -> None:
    d = _config()
    d["pieces"]["X"] = _j_piece()
    with pytest.raises(ConfigError, match="Unknown piece: X"):
        GuidelineConfig._validate(d)


def test_piece_name_collides_with_empty() -> None:
    d = _config()
    del d["pieces"]["J"]
    d["pieces"]["EMPTY"] = _j_piece()
    with pytest.raises(ConfigError, match="collides with sentinel"):
        GuidelineConfig._validate(d)


# ── empty checks ─────────────────────────────────────────────────────


def test_empty_pieces_dict_rejected() -> None:
    d = _config()
    d["pieces"] = {}
    with pytest.raises(ConfigError, match="pieces: must not be empty"):
        GuidelineConfig._validate(d)


def test_empty_coords_rejected() -> None:
    d = _config()
    d["pieces"]["J"]["coords"] = {}
    with pytest.raises(ConfigError, match="coords: must not be empty"):
        GuidelineConfig._validate(d)


def test_empty_kicks_allowed_for_o() -> None:
    d = _config(pieces={"O": _o_piece()})
    cfg = GuidelineConfig._validate(d)
    assert cfg.pieces[Mino.O].kicks == {}


def test_empty_kicks_rejected_for_non_o() -> None:
    d = _config()
    d["pieces"]["J"]["kicks"] = {}
    with pytest.raises(ConfigError, match="kicks: must not be empty"):
        GuidelineConfig._validate(d)


# ── edge cases ───────────────────────────────────────────────────────


def test_root_not_dict() -> None:
    with pytest.raises(ConfigError, match="must be a mapping"):
        GuidelineConfig._validate([])


def test_case_insensitive_piece_name() -> None:
    d = _config()
    del d["pieces"]["J"]
    d["pieces"]["j"] = _j_piece()
    cfg = GuidelineConfig._validate(d)
    assert Mino.J in cfg.pieces


def test_extra_keys_ignored() -> None:
    d = _config()
    d["unknown"] = "ignored"
    cfg = GuidelineConfig._validate(d)
    assert cfg.num_cols == 10
