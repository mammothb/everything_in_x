import pygame
import pytest

from pydownstack.game.actions import Action
from pydownstack.game.settings import Settings
from pydownstack.visual.input_handler import InputHandler


@pytest.fixture
def settings() -> Settings:
    return Settings()


@pytest.fixture
def fast_settings() -> Settings:
    # 50ms ≈ 3 frames, 17ms ≈ 1 frame at 60fps
    return Settings(das_frames=50, arr_frames=17)


@pytest.fixture
def handler(settings: Settings) -> InputHandler:
    return InputHandler(settings)


@pytest.fixture
def fast_handler(fast_settings: Settings) -> InputHandler:
    return InputHandler(fast_settings)


def _keydown(key: int) -> pygame.event.Event:
    return pygame.event.Event(pygame.KEYDOWN, key=key)


def _keyup(key: int) -> pygame.event.Event:
    return pygame.event.Event(pygame.KEYUP, key=key)


def _poll_n(handler: InputHandler, n: int) -> list[Action]:
    """Call poll_actions N times, return all actions emitted."""
    result: list[Action] = []
    for _ in range(n):
        result.extend(handler.poll_actions())
    return result


# ── keybinding ───────────────────────────────────────────────────────


class TestKeybinding:
    def test_known_key_returns_action(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_LEFT)])
        assert handler.poll_actions() == [Action.MOVE_LEFT]

    def test_unknown_key_ignored(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_BACKQUOTE)])
        assert handler.poll_actions() == []

    def test_all_default_bindings_fire(self, settings: Settings) -> None:
        handler = InputHandler(settings)
        for action, key_name in settings.keybindings.items():
            keycode = pygame.key.key_code(key_name)
            handler.push_events([_keydown(keycode)])
            actions = handler.poll_actions()
            assert action in actions, f"{action} / {key_name} not fired"


# ── instant actions ──────────────────────────────────────────────────


class TestInstantActions:
    def test_hard_drop_fires_on_keydown_only(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_SPACE)])
        assert Action.HARD_DROP in handler.poll_actions()
        # No repeats — not in DAS_REPEATABLE, not SOFT_DROP
        assert handler.poll_actions() == []

    def test_rotate_fires_on_keydown_only(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_f)])
        assert Action.ROTATE_CW in handler.poll_actions()
        assert handler.poll_actions() == []

    def test_hold_fires_on_keydown_only(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_s)])
        assert Action.HOLD in handler.poll_actions()
        assert handler.poll_actions() == []

    def test_reset_fires_on_keydown_only(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_F4)])
        assert Action.RESET in handler.poll_actions()
        assert handler.poll_actions() == []

    def test_quit_fires_from_event(self, handler: InputHandler) -> None:
        handler.push_events([pygame.event.Event(pygame.QUIT)])
        assert Action.QUIT in handler.poll_actions()


# ── soft drop (continuous) ───────────────────────────────────────────


class TestSoftDrop:
    def test_soft_drop_fires_every_frame_while_held(
        self, handler: InputHandler
    ) -> None:
        handler.push_events([_keydown(pygame.K_DOWN)])
        # KEYDOWN fires immediately
        first = handler.poll_actions()
        assert Action.SOFT_DROP in first
        # Subsequent polls fire continuously
        for _ in range(5):
            assert Action.SOFT_DROP in handler.poll_actions()
        # KEYUP stops
        handler.push_events([_keyup(pygame.K_DOWN)])
        assert handler.poll_actions() == []

    def test_soft_drop_stops_on_keyup(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_DOWN)])
        assert Action.SOFT_DROP in handler.poll_actions()
        handler.push_events([_keyup(pygame.K_DOWN)])
        assert handler.poll_actions() == []


# ── DAS / ARR ────────────────────────────────────────────────────────


class TestDasArr:
    def test_move_fires_on_keydown(self, fast_handler: InputHandler) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        assert Action.MOVE_LEFT in fast_handler.poll_actions()

    def test_move_repeats_after_das_delay(self, fast_handler: InputHandler) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        fast_handler.poll_actions()  # consume KEYDOWN, counter→1
        # das_frames=3: counter was 1 after consumption.
        # Poll 1: counter=2 (< 3, no fire)
        # Poll 2: counter=3 (≥ 3, fires!)
        actions = _poll_n(fast_handler, 1)
        assert Action.MOVE_LEFT not in actions
        actions = fast_handler.poll_actions()
        assert Action.MOVE_LEFT in actions

    def test_move_repeats_at_arr_rate_after_das(
        self, fast_handler: InputHandler
    ) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        # Consume KEYDOWN + DAS delay (3 frames) + first repeat
        _poll_n(fast_handler, 1)  # KEYDOWN
        _poll_n(fast_handler, 3)  # DAS delay → first repeat at frame 3
        # Now in ARR phase at arr_frames=1: every poll fires
        for _ in range(5):
            assert Action.MOVE_LEFT in fast_handler.poll_actions()

    def test_keyup_stops_das(self, fast_handler: InputHandler) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        fast_handler.poll_actions()  # consume KEYDOWN
        fast_handler.push_events([_keyup(pygame.K_LEFT)])
        # No more repeats
        for _ in range(10):
            assert Action.MOVE_LEFT not in fast_handler.poll_actions()

    def test_newest_direction_wins(
        self, fast_handler: InputHandler
    ) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        assert Action.MOVE_LEFT in fast_handler.poll_actions()
        # Press right — cancels left DAS, starts right
        fast_handler.push_events([_keydown(pygame.K_RIGHT)])
        actions = fast_handler.poll_actions()
        assert Action.MOVE_RIGHT in actions  # KEYDOWN fires immediately
        # Left should NOT repeat — it was cancelled
        _poll_n(fast_handler, 4)  # wait through DAS delay
        actions = fast_handler.poll_actions()
        assert Action.MOVE_RIGHT in actions  # right repeats
        assert Action.MOVE_LEFT not in actions  # left is dead


# ── ordering: events before poll ─────────────────────────────────────


class TestOrdering:
    def test_keydown_fires_before_das_repeat_in_same_frame(
        self, fast_handler: InputHandler
    ) -> None:
        # Press left, wait through DAS, left is repeating
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        _poll_n(fast_handler, 1)  # KEYDOWN
        _poll_n(fast_handler, 3)  # DAS → first repeat of left
        # Press right — cancels left, fires right immediately
        fast_handler.push_events([_keydown(pygame.K_RIGHT)])
        actions = fast_handler.poll_actions()
        assert Action.MOVE_RIGHT in actions  # immediate KEYDOWN
        assert Action.MOVE_LEFT not in actions  # cancelled by right press


# ── edge cases ───────────────────────────────────────────────────────


class TestEdgeCases:
    def test_multiple_keys_same_frame(self, handler: InputHandler) -> None:
        handler.push_events([_keydown(pygame.K_LEFT), _keydown(pygame.K_RIGHT)])
        actions = handler.poll_actions()
        assert Action.MOVE_LEFT in actions
        assert Action.MOVE_RIGHT in actions

    def test_keyup_without_keydown_no_error(self, handler: InputHandler) -> None:
        handler.push_events([_keyup(pygame.K_LEFT)])
        assert handler.poll_actions() == []  # no crash

    def test_poll_without_events_returns_empty(self, handler: InputHandler) -> None:
        assert handler.poll_actions() == []

    def test_unknown_event_type_ignored(self, handler: InputHandler) -> None:
        handler.push_events([pygame.event.Event(pygame.MOUSEMOTION)])
        assert handler.poll_actions() == []

    def test_das_resets_on_second_keydown(self, fast_handler: InputHandler) -> None:
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        fast_handler.poll_actions()  # KEYDOWN, counter→1
        _poll_n(fast_handler, 1)  # counter=2
        # Press left again — resets DAS state to counter=0
        fast_handler.push_events([_keydown(pygame.K_LEFT)])
        fast_handler.poll_actions()  # KEYDOWN again, counter→1
        # Poll 1: counter=2 (< 3, no repeat)
        assert fast_handler.poll_actions() == []
        # Poll 2: counter=3 (≥ 3, first DAS repeat)
        assert fast_handler.poll_actions() == [Action.MOVE_LEFT]
