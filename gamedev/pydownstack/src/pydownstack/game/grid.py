from typing import final

from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D


@final
class Grid:
    def __init__(self, num_cols: int, num_rows: int) -> None:
        self.num_cols = num_cols
        self.num_rows = num_rows
        self._cells: list[list[Mino]] = [
            [Mino.EMPTY] * num_cols for _ in range(num_rows)
        ]

    def __contains__(self, coord: Vector2D) -> bool:
        return 0 <= coord.y < self.num_rows and 0 <= coord.x < self.num_cols

    def __getitem__(self, coord: Vector2D) -> Mino:
        return self._cells[coord.y][coord.x]

    def __setitem__(self, coord: Vector2D, mino: Mino) -> None:
        self._cells[coord.y][coord.x] = mino

    def insert_empty_row(self, y: int) -> None:
        self._cells.insert(y, [Mino.EMPTY] * self.num_cols)

    def insert_bottom_row(self, row: list[Mino]) -> None:
        self._cells.insert(0, row)

    def remove_row(self, y: int) -> None:
        del self._cells[y]

    def remove_top_row(self) -> None:
        del self._cells[-1]
