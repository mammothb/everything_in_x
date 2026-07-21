from pydownstack.domain.rotation import Rotation
from pydownstack.domain.vector import Vector2D
from pydownstack.game.board import Board
from pydownstack.game.config import PieceConfig


def get_cells(piece: PieceConfig, rot: int, origin: Vector2D) -> list[Vector2D]:
    """Returns world-space cell positions in math conventions, x+ = right,
    y+ = up.
    """
    return [Vector2D(x=origin.x + dx, y=origin.y + dy) for dx, dy in piece.coords[rot]]


def ghost_origin(
    piece: PieceConfig, rot: int, origin: Vector2D, board: Board
) -> Vector2D:
    """Drop piece as far down as possible. Returns the lowest valid origin."""
    while True:
        next_origin = Vector2D(x=origin.x, y=origin.y - 1)
        cells = get_cells(piece=piece, rot=rot, origin=next_origin)
        if board.collides(cells=cells):
            return origin
        origin = next_origin


def try_rotation(
    piece: PieceConfig, rot: int, origin: Vector2D, rotation: Rotation, board: Board
) -> tuple[int, Vector2D] | None:
    """Attempts rotation with wall kicks. Returns (new_rot, new_origin) or
    None.
    """
    new_rot = (rot + rotation.value) % 4
    kicks = piece.kicks.get(rotation, {}).get(new_rot, [])
    for offset in kicks:
        new_origin = Vector2D(x=origin.x + offset.x, y=origin.y + offset.y)
        cells = get_cells(piece=piece, rot=new_rot, origin=new_origin)
        if not board.collides(cells=cells):
            return new_rot, new_origin
    return None
