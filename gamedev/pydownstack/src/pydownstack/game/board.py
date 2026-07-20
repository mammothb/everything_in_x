from typing import final

from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D
from pydownstack.game.grid import Grid


@final
class Board:
    def __init__(self, num_cols: int, num_rows: int) -> None:
        self.grid = Grid(num_cols=num_cols, num_rows=num_rows)

    def clear_lines(self, row_indices: list[int]) -> None:
        for y in sorted(row_indices, reverse=True):
            self.grid.remove_row(y)
        for _ in row_indices:
            self.grid.insert_empty_row(self.grid.num_rows - 1)

    def collides(self, cells: list[Vector2D]) -> bool:
        """Checks if the provided `cells` has collision with any cells in the
        board or the walls.
        """
        return any(
            coord not in self.grid or self.grid[coord] != Mino.EMPTY for coord in cells
        )

    def insert_garbage(self, line: list[Mino]) -> None:
        self.grid.insert_bottom_row(line)
        self.grid.remove_top_row()  # board doesn't grow

    def is_top_out(self, cells: list[Vector2D], buffer_rows: int) -> bool:
        return any(coord.y >= self.grid.num_rows - buffer_rows for coord in cells)

    def lock(self, cells: list[Vector2D], mino: Mino) -> list[int]:
        """Write piece cells into the grid. Returns indices of full rows."""
        for coord in cells:
            self.grid[coord] = mino
        return [y for y in range(self.grid.num_rows) if self._line_is_full(y)]

    def _line_is_full(self, y: int) -> bool:
        return all(
            self.grid[Vector2D(x, y)] != Mino.EMPTY for x in range(self.grid.num_cols)
        )
