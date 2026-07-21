from collections import deque
from typing import final, override

from pydownstack.domain.mino import Mino
from pydownstack.domain.rotation import Rotation
from pydownstack.domain.vector import Vector2D
from pydownstack.game.actions import Action, GamePhase
from pydownstack.game.bag import Bag
from pydownstack.game.board import Board
from pydownstack.game.config import GuidelineConfig, PieceConfig
from pydownstack.game.events import (
    GameEvent,
    GameOver,
    HardDropped,
    LineCleared,
    PieceLocked,
    PieceMoved,
    PieceRotated,
)
from pydownstack.game.game_state import GameState
from pydownstack.game.piece import get_cells, try_rotation
from pydownstack.inbound_ports import GameEnginePort


@final
class GameEngine(GameEnginePort):
    def __init__(
        self, config: GuidelineConfig, gravity_frames: int, seed: int | None = None
    ) -> None:
        self._config = config
        self._gravity_frames = gravity_frames
        self._bag = Bag(config=config, seed=seed)
        self._board = Board(num_cols=config.num_cols, num_rows=config.num_rows)
        self._next_queue: deque[Mino] = deque()
        for _ in range(config.num_previews):
            self._next_queue.append(next(self._bag))
        self._hold: Mino | None = None
        self._hold_used = False
        self._score = 0
        self._lines_cleared = 0
        self._gravity_counter = 0
        self._spawn()

    @override
    def apply_action(self, action: Action) -> list[GameEvent]:
        if action == Action.RESET:
            self.reset()
            return []
        if self._phase != GamePhase.PLAYING:
            return []

        events: list[GameEvent] = []
        match action:
            case Action.MOVE_LEFT | Action.MOVE_RIGHT:
                self._move(action, events)
            case Action.ROTATE_CW | Action.ROTATE_CCW:
                self._rotate(action, events)
            case Action.SOFT_DROP:
                self._soft_drop()
            case Action.HARD_DROP:
                self._hard_drop(events)
            case Action.HOLD:
                self._hold_piece()
            case Action.PAUSE:
                self._phase = GamePhase.PAUSED
            case Action.QUIT:
                pass
        return events

    @override
    def reset(self) -> None:
        self.__init__(config=self._config, gravity_frames=self._gravity_frames)

    @override
    def tick(self) -> list[GameEvent]:
        if self._phase != GamePhase.PLAYING:
            return []
        self._gravity_counter += 1
        if self._gravity_counter >= self._gravity_frames:
            self._gravity_counter = 0
            return self._gravity_drop()
        return []

    def get_state(self) -> GameState:
        return GameState(
            board=self._board,
            curr_piece=self._curr_piece,
            curr_rot=self._curr_rot,
            curr_origin=self._curr_origin,
            hold_piece=self._hold,
            hold_used=self._hold_used,
            next_queue=list(self._next_queue),
            bag=self._bag,
            score=self._score,
            lines_cleared=self._lines_cleared,
            phase=self._phase,
        )

    @property
    def _piece_config(self) -> PieceConfig:
        return self._config.pieces[self._curr_piece]

    def _spawn(self) -> None:
        self._curr_piece = self._next_queue.popleft()
        self._next_queue.append(next(self._bag))
        self._curr_rot = 0
        self._curr_origin = self._config.pieces[self._curr_piece].origin
        cells = get_cells(piece=self._piece_config, rot=0, origin=self._curr_origin)
        if self._board.collides(cells):
            self._phase = GamePhase.GAME_OVER
        else:
            self._phase = GamePhase.PLAYING

    def _move(self, action: Action, events: list[GameEvent]) -> None:
        dx = -1 if action == Action.MOVE_LEFT else 1
        origin = Vector2D(x=self._curr_origin.x + dx, y=self._curr_origin.y)
        cells = get_cells(piece=self._piece_config, rot=self._curr_rot, origin=origin)
        if not self._board.collides(cells):
            self._curr_origin = origin
            events.append(PieceMoved())

    def _rotate(self, action: Action, events: list[GameEvent]) -> None:
        rotation = Rotation.CW if action == Action.ROTATE_CW else Rotation.CCW
        result = try_rotation(
            piece=self._piece_config,
            rot=self._curr_rot,
            origin=self._curr_origin,
            rotation=rotation,
            board=self._board,
        )
        if result is not None:
            self._curr_rot, self._curr_origin = result
            events.append(PieceRotated())

    def _soft_drop(self) -> None:
        origin = Vector2D(x=self._curr_origin.x, y=self._curr_origin.y - 1)
        cells = get_cells(piece=self._piece_config, rot=self._curr_rot, origin=origin)
        if not self._board.collides(cells):
            self._curr_origin = origin

    def _hard_drop(self, events: list[GameEvent]) -> None:
        distance = 0
        while True:
            origin = Vector2D(
                x=self._curr_origin.x, y=self._curr_origin.y - distance - 1
            )
            cells = get_cells(
                piece=self._piece_config, rot=self._curr_rot, origin=origin
            )
            if self._board.collides(cells):
                break
            distance += 1
        self._curr_origin = Vector2D(
            x=self._curr_origin.x, y=self._curr_origin.y - distance
        )
        events.append(HardDropped(distance=distance))
        self._lock(events)

    def _gravity_drop(self) -> list[GameEvent]:
        events: list[GameEvent] = []
        origin = Vector2D(x=self._curr_origin.x, y=self._curr_origin.y - 1)
        cells = get_cells(piece=self._piece_config, rot=self._curr_rot, origin=origin)
        if self._board.collides(cells):
            self._lock(events)
        else:
            self._curr_origin = origin
        return events

    def _lock(self, events: list[GameEvent]) -> None:
        cells = get_cells(
            piece=self._piece_config, rot=self._curr_rot, origin=self._curr_origin
        )
        full_rows = self._board.lock(cells=cells, mino=self._curr_piece)
        cleared = len(full_rows)
        events.append(PieceLocked(cleared_lines=cleared))
        if full_rows:
            self._board.clear_lines(full_rows)
            events.append(LineCleared(count=cleared, was_tetris=(cleared == 4)))
            self._lines_cleared += cleared
        self._hold_used = False
        self._spawn()
        if self._phase == GamePhase.GAME_OVER:
            events.append(GameOver())

    def _hold_piece(self) -> None:
        if self._hold_used:
            return
        self._hold_used = True
        if self._hold is None:
            self._hold = self._curr_piece
            self._spawn()
        else:
            self._hold, to_spawn = self._curr_piece, self._hold
            self._curr_piece = to_spawn
            self._curr_rot = 0
            self._curr_origin = self._config.pieces[self._curr_piece].origin
