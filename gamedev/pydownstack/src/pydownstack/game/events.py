from collections.abc import Callable
from dataclasses import dataclass
from typing import Any


class GameEvent: ...


@dataclass
class LineCleared(GameEvent):
    count: int  # 1-4
    was_tetris: bool = False


@dataclass
class HardDropped(GameEvent):
    distance: int  # cells dropped


@dataclass
class PieceLocked(GameEvent):
    cleared_lines: int  # 0 if no lines cleared this lock


class PieceMoved(GameEvent): ...


class PieceRotated(GameEvent): ...


class GameOver(GameEvent): ...


class EventBus:
    def __init__(self) -> None:
        self._event_to_handlers: dict[type[GameEvent], list[Callable[[Any], None]]] = {}

    def subscribe[T: GameEvent](
        self, event_type: type[T], handler: Callable[[T], None]
    ) -> None:
        self._event_to_handlers.setdefault(event_type, []).append(handler)

    def emit(self, event: GameEvent) -> None:
        for handler in self._event_to_handlers.get(type(event), []):
            handler(event)
