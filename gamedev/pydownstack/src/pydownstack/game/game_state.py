from dataclasses import dataclass

from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D
from pydownstack.game.actions import GamePhase
from pydownstack.game.bag import Bag
from pydownstack.game.board import Board


@dataclass(frozen=True)
class GameState:
    board: Board
    curr_piece: Mino | None
    curr_rot: int
    curr_origin: Vector2D
    ghost_origin: Vector2D
    hold_piece: Mino | None
    hold_used: bool
    next_queue: list[Mino]
    bag: Bag
    score: int
    lines_cleared: int
    phase: GamePhase
