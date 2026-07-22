from typing import final, override

import pygame

from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D
from pydownstack.game.config import GuidelineConfig, PieceConfig
from pydownstack.game.game_state import GameState
from pydownstack.outbound_ports import RendererPort

_COLORS: dict[Mino, pygame.Color] = {
    Mino.J: pygame.Color("#8193FF"),
    Mino.L: pygame.Color("#FFCC90"),
    Mino.S: pygame.Color("#91D9A6"),
    Mino.Z: pygame.Color("#EA6989"),
    Mino.T: pygame.Color("#E486FA"),
    Mino.I: pygame.Color("#73BFEA"),
    Mino.O: pygame.Color("#FFF978"),
    Mino.EMPTY: pygame.Color("#222222"),
    Mino.GARBAGE: pygame.Color("#D5D7E2"),
}

_CELL = 32
_BOARD_X = 16
_BOARD_Y = 16
_VISIBLE = 20
_SIDE_X = _BOARD_X + 10 * _CELL + 8
_HOLD_Y = _BOARD_Y
_NEXT_Y = _HOLD_Y + 5 * _CELL + 16
_HUD_Y = _NEXT_Y + 6 * 3 * _CELL + 16

_BG = pygame.Color("#0F0F23")
_GRID = pygame.Color("#2A2A4A")
_TEXT = pygame.Color("#CCCCCC")


def _darken(c: pygame.Color) -> pygame.Color:
    return pygame.Color(c.r // 3, c.g // 3, c.b // 3)


@final
class PygameRenderer(RendererPort):
    """Renders GameState to a pygame window. Single Y-flip boundary."""

    def __init__(self, config: GuidelineConfig) -> None:
        self._config = config
        self._screen = pygame.display.set_mode((_SIDE_X + 144, 704))
        self._font = pygame.font.Font(None, 22)
        self._small = pygame.font.Font(None, 16)

    @override
    def draw_frame(self, state: GameState) -> None:
        self._screen.fill(_BG)
        self._draw_board(state)
        if state.curr_piece is not None:
            self._draw_ghost(state)
            self._draw_active_piece(state)
        self._draw_hold(state)
        self._draw_next(state)
        self._draw_hud(state)
        pygame.display.flip()

    # ── board ──────────────────────────────────────────────────────

    def _draw_board(self, state: GameState) -> None:
        for y in range(_VISIBLE):
            for x in range(10):
                mino = state.board.grid[Vector2D(x, y)]
                rect = self._cell_rect(x, y)
                if mino == Mino.EMPTY:
                    pygame.draw.rect(self._screen, _GRID, rect, 1)
                else:
                    pygame.draw.rect(self._screen, _COLORS[mino], rect)
                    pygame.draw.rect(
                        self._screen, _darken(_COLORS[mino]), rect.inflate(-4, -4)
                    )

    # ── active / ghost ─────────────────────────────────────────────

    def _draw_active_piece(self, state: GameState) -> None:
        piece = self._config.pieces[state.curr_piece]
        color = _COLORS[state.curr_piece]
        for dx, dy in piece.coords[state.curr_rot]:
            x = state.curr_origin.x + dx
            y = state.curr_origin.y + dy
            if 0 <= y < _VISIBLE:
                rect = self._cell_rect(x, y)
                pygame.draw.rect(self._screen, color, rect)
                pygame.draw.rect(self._screen, _darken(color), rect.inflate(-4, -4))

    def _draw_ghost(self, state: GameState) -> None:
        piece = self._config.pieces[state.curr_piece]
        color = _COLORS[state.curr_piece]
        ghost = pygame.Color(color.r, color.g, color.b, 60)
        for dx, dy in piece.coords[state.curr_rot]:
            x = state.ghost_origin.x + dx
            y = state.ghost_origin.y + dy
            if 0 <= y < _VISIBLE:
                rect = self._cell_rect(x, y)
                pygame.draw.rect(self._screen, ghost, rect)
                pygame.draw.rect(self._screen, ghost, rect, 2)

    # ── hold ───────────────────────────────────────────────────────

    def _draw_hold(self, state: GameState) -> None:
        self._label("HOLD", _SIDE_X, _HOLD_Y)
        if state.hold_piece is not None:
            piece = self._config.pieces[state.hold_piece]
            color = _COLORS[state.hold_piece]
            self._draw_mini_piece(piece, color, _SIDE_X + 8, _HOLD_Y + 20)

    # ── next queue ─────────────────────────────────────────────────

    def _draw_next(self, state: GameState) -> None:
        self._label("NEXT", _SIDE_X, _NEXT_Y)
        for i, mino in enumerate(state.next_queue[:5]):
            piece = self._config.pieces[mino]
            color = _COLORS[mino]
            y_off = _NEXT_Y + 20 + i * 3 * _CELL
            self._draw_mini_piece(piece, color, _SIDE_X + 8, y_off)

    # ── HUD ────────────────────────────────────────────────────────

    def _draw_hud(self, state: GameState) -> None:
        y = _HUD_Y
        self._label("SCORE", _SIDE_X, y)
        y += 18
        self._value(str(state.score), _SIDE_X, y)
        y += 28
        self._label("LINES", _SIDE_X, y)
        y += 18
        self._value(str(state.lines_cleared), _SIDE_X, y)

    # ── helpers ────────────────────────────────────────────────────

    def _cell_rect(self, col: int, row: int) -> pygame.Rect:
        """Y-flip: row 0 (bottom) → top of board area."""
        return pygame.Rect(
            _BOARD_X + col * _CELL,
            _BOARD_Y + (_VISIBLE - 1 - row) * _CELL,
            _CELL,
            _CELL,
        )

    def _draw_mini_piece(
        self, piece: PieceConfig, color: pygame.Color, ox: int, oy: int
    ) -> None:
        """Draw a small piece preview (3×3 or 4×4 cells at reduced scale)."""
        s = _CELL // 2
        h = piece.width - 1  # max Y in bounding box
        for dx, dy in piece.coords[0]:
            rx = ox + dx * s
            ry = oy + (h - dy) * s  # flip Y: math convention -> screen
            rect = pygame.Rect(rx, ry, s, s)
            pygame.draw.rect(self._screen, color, rect)

    def _label(self, text: str, x: int, y: int) -> None:
        surf = self._small.render(text, True, _TEXT)
        self._screen.blit(surf, (x, y))

    def _value(self, text: str, x: int, y: int) -> None:
        surf = self._font.render(text, True, _TEXT)
        self._screen.blit(surf, (x, y))
