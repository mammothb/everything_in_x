from dataclasses import dataclass

from pydownstack.game.game_state import GameState


@dataclass(frozen=True)
class FrameData:
    """Immutable snapshot of everything the renderer needs for one frame."""

    state: GameState
    score: int
    lines: int
