import pytest

from pydownstack.domain.mino import Mino
from pydownstack.game.bag import Bag
from pydownstack.game.config import GuidelineConfig


@pytest.fixture
def bag(guideline_config: GuidelineConfig) -> Bag:
    return Bag(guideline_config, seed=42)


# ── properties ───────────────────────────────────────────────────────


def test_size_matches_piece_count(guideline_config: GuidelineConfig) -> None:
    bag = Bag(guideline_config, seed=0)
    assert bag.size == len(guideline_config.pieces) == 7


def test_only_playable_minos_emitted(bag: Bag) -> None:
    for _ in range(70):
        piece = next(bag)
        assert piece not in (Mino.EMPTY, Mino.GARBAGE)


# ── 7-bag guarantee ──────────────────────────────────────────────────


def test_first_seven_are_permutation(bag: Bag) -> None:
    pieces = [next(bag) for _ in range(7)]
    assert len(set(pieces)) == 7


def test_second_seven_are_permutation(bag: Bag) -> None:
    _ = [next(bag) for _ in range(7)]
    pieces = [next(bag) for _ in range(7)]
    assert len(set(pieces)) == 7


def test_third_seven_are_permutation(bag: Bag) -> None:
    _ = [next(bag) for _ in range(14)]
    pieces = [next(bag) for _ in range(7)]
    assert len(set(pieces)) == 7


def test_ten_bags_all_permutations(bag: Bag) -> None:
    for _ in range(10):
        pieces = [next(bag) for _ in range(7)]
        assert len(set(pieces)) == 7


# ── determinism ──────────────────────────────────────────────────────


def test_same_seed_same_sequence(guideline_config: GuidelineConfig) -> None:
    bag1 = Bag(guideline_config, seed=123)
    bag2 = Bag(guideline_config, seed=123)
    assert [next(bag1) for _ in range(14)] == [next(bag2) for _ in range(14)]


def test_different_seeds_diverge(guideline_config: GuidelineConfig) -> None:
    bag1 = Bag(guideline_config, seed=1)
    bag2 = Bag(guideline_config, seed=2)
    seq1 = [next(bag1) for _ in range(7)]
    seq2 = [next(bag2) for _ in range(7)]
    assert seq1 != seq2  # 1 in 5040 false positive


# ── iteration protocol ───────────────────────────────────────────────


def test_iter_yields_full_bag(bag: Bag) -> None:
    pieces: list[Mino] = []
    for i, piece in enumerate(bag):
        pieces.append(piece)
        if i >= 6:
            break
    assert len(set(pieces)) == 7


def test_iter_resets_position(bag: Bag) -> None:
    _ = [next(bag) for _ in range(3)]
    pieces: list[Mino] = []
    for i, piece in enumerate(bag):
        pieces.append(piece)
        if i >= 6:
            break
    assert len(set(pieces)) == 7
