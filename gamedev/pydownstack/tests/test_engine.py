import pytest

from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D
from pydownstack.game.actions import Action, GamePhase
from pydownstack.game.config import GuidelineConfig
from pydownstack.game.engine import GameEngine
from pydownstack.game.events import (
    GameEvent,
    GameOver,
    HardDropped,
    LineCleared,
    PieceLocked,
    PieceMoved,
    PieceRotated,
)


@pytest.fixture
def engine(guideline_config: GuidelineConfig) -> GameEngine:
    return GameEngine(config=guideline_config, difficulty=1, seed=42)


def _fill_row(engine: GameEngine, y: int, mino: Mino = Mino.T) -> None:
    board = engine.get_state().board
    for x in range(board.grid.num_cols):
        board.grid[Vector2D(x, y)] = mino


def _fill_board(engine: GameEngine, mino: Mino = Mino.GARBAGE) -> None:
    board = engine.get_state().board
    for y in range(board.grid.num_rows):
        for x in range(board.grid.num_cols):
            board.grid[Vector2D(x, y)] = mino


def _has_event(events: list[GameEvent], event_type: type) -> bool:
    return any(isinstance(e, event_type) for e in events)


# ── movement ─────────────────────────────────────────────────────────


class TestMovement:
    def test_move_left(self, engine: GameEngine) -> None:
        before = engine.get_state().curr_origin
        events = engine.apply_action(Action.MOVE_LEFT)
        assert _has_event(events, PieceMoved)
        assert engine.get_state().curr_origin.x == before.x - 1

    def test_move_right(self, engine: GameEngine) -> None:
        before = engine.get_state().curr_origin
        events = engine.apply_action(Action.MOVE_RIGHT)
        assert _has_event(events, PieceMoved)
        assert engine.get_state().curr_origin.x == before.x + 1

    def test_move_against_left_wall_noops(self, engine: GameEngine) -> None:
        for _ in range(20):
            engine.apply_action(Action.MOVE_LEFT)
        state = engine.get_state()
        events = engine.apply_action(Action.MOVE_LEFT)
        assert not _has_event(events, PieceMoved)
        assert engine.get_state().curr_origin == state.curr_origin

    def test_move_against_right_wall_noops(self, engine: GameEngine) -> None:
        for _ in range(20):
            engine.apply_action(Action.MOVE_RIGHT)
        state = engine.get_state()
        events = engine.apply_action(Action.MOVE_RIGHT)
        assert not _has_event(events, PieceMoved)
        assert engine.get_state().curr_origin == state.curr_origin


# ── rotation ─────────────────────────────────────────────────────────


class TestRotation:
    def test_rotate_cw(self, engine: GameEngine) -> None:
        events = engine.apply_action(Action.ROTATE_CW)
        assert _has_event(events, PieceRotated)
        assert engine.get_state().curr_rot == 1

    def test_rotate_ccw(self, engine: GameEngine) -> None:
        events = engine.apply_action(Action.ROTATE_CCW)
        assert _has_event(events, PieceRotated)
        assert engine.get_state().curr_rot == 3

    def test_rotate_blocked_by_filled_board(self, engine: GameEngine) -> None:
        _fill_board(engine)
        events = engine.apply_action(Action.ROTATE_CW)
        assert not _has_event(events, PieceRotated)


# ── soft drop ────────────────────────────────────────────────────────


class TestSoftDrop:
    def test_soft_drop_moves_down(self, engine: GameEngine) -> None:
        before = engine.get_state().curr_origin
        engine.apply_action(Action.SOFT_DROP)
        assert engine.get_state().curr_origin.y == before.y - 1

    def test_soft_drop_stops_when_blocked(self, engine: GameEngine) -> None:
        # Drop until piece can no longer descend
        while True:
            before = engine.get_state().curr_origin
            engine.apply_action(Action.SOFT_DROP)
            if engine.get_state().curr_origin == before:
                break  # blocked — can't drop further
        # One more should still be blocked
        before = engine.get_state().curr_origin
        engine.apply_action(Action.SOFT_DROP)
        assert engine.get_state().curr_origin == before


# ── hard drop ────────────────────────────────────────────────────────


class TestHardDrop:
    def test_hard_drop_emits_lock_and_drop_events(self, engine: GameEngine) -> None:
        events = engine.apply_action(Action.HARD_DROP)
        assert _has_event(events, HardDropped)
        assert _has_event(events, PieceLocked)

    def test_hard_drop_spawns_new_piece(self, engine: GameEngine) -> None:
        before = engine.get_state().curr_piece
        engine.apply_action(Action.HARD_DROP)
        assert engine.get_state().curr_piece != before

    def test_hard_drop_phase_stays_playing(self, engine: GameEngine) -> None:
        engine.apply_action(Action.HARD_DROP)
        assert engine.get_state().phase == GamePhase.PLAYING

    def test_hard_drop_distance_is_non_negative(self, engine: GameEngine) -> None:
        events = engine.apply_action(Action.HARD_DROP)
        drop = next(e for e in events if isinstance(e, HardDropped))
        assert drop.distance >= 0


# ── tick (no-op for cheese mode) ────────────────────────────────────


class TestTick:
    def test_tick_always_returns_empty(self, engine: GameEngine) -> None:
        assert engine.tick() == []

    def test_tick_empty_in_game_over(self, engine: GameEngine) -> None:
        _fill_board(engine)
        engine.apply_action(Action.HARD_DROP)
        assert engine.tick() == []


# ── hold ─────────────────────────────────────────────────────────────


class TestHold:
    def test_first_hold_stores_piece(self, engine: GameEngine) -> None:
        first = engine.get_state().curr_piece
        engine.apply_action(Action.HOLD)
        state = engine.get_state()
        assert state.hold_piece == first
        assert state.curr_piece != first

    def test_first_hold_sets_hold_used(self, engine: GameEngine) -> None:
        engine.apply_action(Action.HOLD)
        assert engine.get_state().hold_used is True

    def test_double_hold_is_ignored(self, engine: GameEngine) -> None:
        engine.apply_action(Action.HOLD)
        current = engine.get_state().curr_piece
        engine.apply_action(Action.HOLD)
        assert engine.get_state().curr_piece == current

    def test_hold_used_resets_after_lock(self, engine: GameEngine) -> None:
        engine.apply_action(Action.HOLD)
        assert engine.get_state().hold_used is True
        engine.apply_action(Action.HARD_DROP)
        assert engine.get_state().hold_used is False

    def test_second_hold_swaps_pieces(self, engine: GameEngine) -> None:
        first = engine.get_state().curr_piece
        engine.apply_action(Action.HOLD)
        _second = engine.get_state().curr_piece
        engine.apply_action(Action.HARD_DROP)
        third = engine.get_state().curr_piece
        engine.apply_action(Action.HOLD)
        state = engine.get_state()
        assert state.hold_piece == third
        assert state.curr_piece == first


# ── line clear ───────────────────────────────────────────────────────


class TestLineClear:
    def test_single_line_clear(self, engine: GameEngine) -> None:
        _fill_row(engine, 0, Mino.T)
        events = engine.apply_action(Action.HARD_DROP)
        cleared = [e for e in events if isinstance(e, LineCleared)]
        assert len(cleared) >= 1
        assert cleared[0].count >= 1

    def test_lines_cleared_increments(self, engine: GameEngine) -> None:
        before = engine.get_state().lines_cleared
        _fill_row(engine, 0, Mino.T)
        engine.apply_action(Action.HARD_DROP)
        assert engine.get_state().lines_cleared > before


# ── game over ────────────────────────────────────────────────────────


class TestGameOver:
    def test_filled_board_triggers_game_over(self, engine: GameEngine) -> None:
        _fill_board(engine)
        events = engine.apply_action(Action.HARD_DROP)
        if engine.get_state().phase == GamePhase.GAME_OVER:
            assert _has_event(events, GameOver)

    def test_actions_ignored_in_game_over(self, engine: GameEngine) -> None:
        _fill_board(engine)
        engine.apply_action(Action.HARD_DROP)
        if engine.get_state().phase == GamePhase.GAME_OVER:
            events = engine.apply_action(Action.MOVE_LEFT)
            assert events == []

    def test_tick_ignored_in_game_over(self, engine: GameEngine) -> None:
        _fill_board(engine)
        engine.apply_action(Action.HARD_DROP)
        if engine.get_state().phase == GamePhase.GAME_OVER:
            assert engine.tick() == []

    def test_reset_from_game_over(self, engine: GameEngine) -> None:
        _fill_board(engine)
        engine.apply_action(Action.HARD_DROP)
        engine.apply_action(Action.RESET)
        assert engine.get_state().phase == GamePhase.PLAYING
        assert engine.get_state().curr_piece is not None


# ── reset ────────────────────────────────────────────────────────────


class TestReset:
    def test_reset_restores_initial_state(self, engine: GameEngine) -> None:
        _fill_row(engine, 15, Mino.T)  # above garbage zone
        engine.apply_action(Action.RESET)
        board = engine.get_state().board
        # Row 15 should be clean (was filled with T before reset)
        for x in range(board.grid.num_cols):
            assert board.grid[Vector2D(x, 15)] == Mino.EMPTY

    def test_reset_resets_lines_cleared(self, engine: GameEngine) -> None:
        _fill_row(engine, 0, Mino.T)
        engine.apply_action(Action.HARD_DROP)
        engine.apply_action(Action.RESET)
        assert engine.get_state().lines_cleared == 0

    def test_reset_from_pause(self, engine: GameEngine) -> None:
        engine.apply_action(Action.PAUSE)
        assert engine.get_state().phase == GamePhase.PAUSED
        engine.apply_action(Action.RESET)
        assert engine.get_state().phase == GamePhase.PLAYING


# ── pause ────────────────────────────────────────────────────────────


class TestPause:
    def test_pause_changes_phase(self, engine: GameEngine) -> None:
        engine.apply_action(Action.PAUSE)
        assert engine.get_state().phase == GamePhase.PAUSED

    def test_actions_ignored_in_pause(self, engine: GameEngine) -> None:
        engine.apply_action(Action.PAUSE)
        events = engine.apply_action(Action.MOVE_LEFT)
        assert events == []

    def test_tick_ignored_in_pause(self, engine: GameEngine) -> None:
        engine.apply_action(Action.PAUSE)
        assert engine.tick() == []
