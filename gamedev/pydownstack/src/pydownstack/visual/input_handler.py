from dataclasses import dataclass
from enum import Enum, auto
from typing import final, override

import pygame

from pydownstack.game.actions import Action
from pydownstack.game.settings import Settings
from pydownstack.outbound_ports import InputPort

# Horizontal movement uses DAS/ARR. Soft drop is continuous (fires every frame).
FPS = 60

_DAS_REPEATABLE: set[Action] = {Action.MOVE_LEFT, Action.MOVE_RIGHT}


class _DasPhase(Enum):
    DAS = auto()
    ARR = auto()


@dataclass
class _DasState:
    phase: _DasPhase = _DasPhase.DAS
    counter: int = 0


@final
class InputHandler(InputPort):
    """Owns pygame event pump, keybinding map, and DAS/ARR state."""

    def __init__(self, settings: Settings) -> None:
        self._key_to_action: dict[int, Action] = {}
        for action, key_name in settings.keybindings.items():
            self._key_to_action[pygame.key.key_code(key_name)] = action

        self._das_frames = max(1, settings.das_frames * FPS // 1000)
        self._arr_frames = max(1, settings.arr_frames * FPS // 1000)

        self._das_states: dict[Action, _DasState] = {}
        self._keys_held: set[Action] = set()
        self._pending: list[Action] = []

    @override
    def poll_actions(self) -> list[Action]:
        for action in self._keys_held:
            if action == Action.SOFT_DROP:
                self._pending.append(action)
            elif action in _DAS_REPEATABLE:
                das = self._das_states[action]
                das.counter += 1
                if das.phase == _DasPhase.DAS:
                    if das.counter >= self._das_frames:
                        self._pending.append(action)
                        das.phase = _DasPhase.ARR
                        das.counter = 0
                else:
                    if das.counter >= self._arr_frames:
                        self._pending.append(action)
                        das.counter = 0

        result = self._pending
        self._pending = []
        return result

    def push_events(self, events: list[pygame.event.Event]) -> None:
        for event in events:
            match event.type:
                case pygame.QUIT:
                    self._pending.append(Action.QUIT)
                case pygame.KEYDOWN:
                    self._handle_key_down(event.key)
                case pygame.KEYUP:
                    self._handle_key_up(event.key)
                case _:
                    pass

    def _handle_key_down(self, key: int) -> None:
        action = self._key_to_action.get(key)
        if action is None:
            return
        self._pending.append(action)
        if action in _DAS_REPEATABLE:
            self._keys_held.add(action)
            self._das_states[action] = _DasState()
        elif action == Action.SOFT_DROP:
            self._keys_held.add(action)  # continuous, no DAS

    def _handle_key_up(self, key: int) -> None:
        action = self._key_to_action.get(key)
        if action is not None:
            self._keys_held.discard(action)
            self._das_states.pop(action, None)
