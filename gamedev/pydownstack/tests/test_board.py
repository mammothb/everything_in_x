from pydownstack.domain.mino import Mino
from pydownstack.domain.vector import Vector2D
from pydownstack.game.board import Board

# ── helpers ──────────────────────────────────────────────────────────


def _fill_row(board: Board, y: int, mino: Mino = Mino.T) -> None:
    for x in range(board.grid.num_cols):
        board.grid[Vector2D(x, y)] = mino


def _row_is(board: Board, y: int, mino: Mino) -> bool:
    return all(board.grid[Vector2D(x, y)] == mino for x in range(board.grid.num_cols))


# ── collides ─────────────────────────────────────────────────────────


class TestCollides:
    def test_empty_board_no_collision(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        assert not board.collides([Vector2D(5, 10), Vector2D(6, 10)])

    def test_out_of_bounds_is_collision(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        assert board.collides([Vector2D(-1, 5)])
        assert board.collides([Vector2D(10, 5)])
        assert board.collides([Vector2D(5, -1)])
        assert board.collides([Vector2D(5, 20)])

    def test_occupied_cell_is_collision(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        board.grid[Vector2D(5, 10)] = Mino.T
        assert board.collides([Vector2D(5, 10)])

    def test_partial_overlap_is_collision(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        board.grid[Vector2D(5, 10)] = Mino.T
        assert board.collides([Vector2D(5, 10), Vector2D(6, 10)])

    def test_all_empty_in_bounds_no_collision(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        assert not board.collides([Vector2D(0, 0), Vector2D(9, 19)])


# ── lock ─────────────────────────────────────────────────────────────


class TestLock:
    def test_writes_cells_to_grid(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        cells = [Vector2D(3, 5), Vector2D(4, 5)]
        board.lock(cells, Mino.S)
        assert board.grid[Vector2D(3, 5)] == Mino.S
        assert board.grid[Vector2D(4, 5)] == Mino.S

    def test_no_full_row_returns_empty(self) -> None:
        board = Board(num_cols=10, num_rows=20)
        result = board.lock([Vector2D(3, 5)], Mino.T)
        assert result == []

    def test_completes_row_returns_index(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        for x in range(3):
            board.grid[Vector2D(x, 2)] = Mino.L
        result = board.lock([Vector2D(3, 2)], Mino.T)
        assert result == [2]

    def test_completes_multiple_rows(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 1, Mino.L)
        for x in range(3):
            board.grid[Vector2D(x, 3)] = Mino.S
        result = board.lock([Vector2D(3, 3)], Mino.T)
        assert sorted(result) == [1, 3]

    def test_already_full_row_still_returned(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 2, Mino.J)
        # Lock unrelated piece — row 2 was already full, still returned
        result = board.lock([Vector2D(0, 4)], Mino.Z)
        assert result == [2]


# ── clear_lines ──────────────────────────────────────────────────────


class TestClearLines:
    def test_single_line_cleared_above_falls(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 0, Mino.I)
        _fill_row(board, 1, Mino.J)
        _fill_row(board, 2, Mino.L)
        _fill_row(board, 3, Mino.S)  # cleared
        _fill_row(board, 4, Mino.T)
        _fill_row(board, 5, Mino.Z)

        board.clear_lines([3])

        # rows below cleared are untouched
        assert _row_is(board, 0, Mino.I)
        assert _row_is(board, 1, Mino.J)
        assert _row_is(board, 2, Mino.L)
        # rows above fall down
        assert _row_is(board, 3, Mino.T)
        assert _row_is(board, 4, Mino.Z)
        # new empty row at top
        assert _row_is(board, 5, Mino.EMPTY)

    def test_multiple_lines_cleared(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 2, Mino.T)
        _fill_row(board, 4, Mino.S)

        board.clear_lines([2, 4])

        assert _row_is(board, 4, Mino.EMPTY)
        assert _row_is(board, 5, Mino.EMPTY)

    def test_clear_bottom_row(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 0, Mino.T)
        _fill_row(board, 3, Mino.S)

        board.clear_lines([0])

        assert _row_is(board, 0, Mino.EMPTY)
        assert _row_is(board, 2, Mino.S)  # was row 3, fell to 2

    def test_clear_top_row(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 5, Mino.T)

        board.clear_lines([5])

        assert _row_is(board, 5, Mino.EMPTY)

    def test_clear_consecutive_lines(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 1, Mino.I)
        _fill_row(board, 2, Mino.J)
        _fill_row(board, 3, Mino.L)
        _fill_row(board, 4, Mino.S)

        board.clear_lines([2, 3])

        # rows below stay
        assert _row_is(board, 0, Mino.EMPTY)
        assert _row_is(board, 1, Mino.I)
        # row 4 falls to row 2
        assert _row_is(board, 2, Mino.S)
        # top two are empty
        assert _row_is(board, 3, Mino.EMPTY)
        assert _row_is(board, 4, Mino.EMPTY)
        assert _row_is(board, 5, Mino.EMPTY)

    def test_clearing_no_rows_noop(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 2, Mino.T)
        board.clear_lines([])
        assert _row_is(board, 2, Mino.T)


# ── is_top_out ───────────────────────────────────────────────────────


class TestIsTopOut:
    def test_visible_cells_no_top_out(self) -> None:
        board = Board(num_cols=10, num_rows=6)  # 4 visible, 2 buffer
        cells = [Vector2D(3, 0), Vector2D(3, 3)]
        assert not board.is_top_out(cells, buffer_rows=2)

    def test_cell_in_buffer_is_top_out(self) -> None:
        board = Board(num_cols=10, num_rows=6)
        cells = [Vector2D(3, 4)]  # row 4 is first buffer row
        assert board.is_top_out(cells, buffer_rows=2)

    def test_partial_buffer_is_top_out(self) -> None:
        board = Board(num_cols=10, num_rows=6)
        cells = [Vector2D(3, 3), Vector2D(3, 4)]
        assert board.is_top_out(cells, buffer_rows=2)

    def test_top_row_is_top_out(self) -> None:
        board = Board(num_cols=10, num_rows=6)
        cells = [Vector2D(3, 5)]  # top row, definitely buffer
        assert board.is_top_out(cells, buffer_rows=2)


# ── insert_garbage ───────────────────────────────────────────────────


class TestInsertGarbage:
    def test_garbage_inserts_at_bottom(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        board.grid[Vector2D(1, 2)] = Mino.T

        garbage = [Mino.GARBAGE, Mino.EMPTY, Mino.GARBAGE, Mino.EMPTY]
        board.insert_garbage(garbage)

        assert board.grid[Vector2D(0, 0)] == Mino.GARBAGE
        assert board.grid[Vector2D(1, 0)] == Mino.EMPTY
        assert board.grid[Vector2D(2, 0)] == Mino.GARBAGE

    def test_garbage_pushes_existing_blocks_up(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        board.grid[Vector2D(1, 2)] = Mino.T

        garbage = [Mino.GARBAGE] * 4
        board.insert_garbage(garbage)

        # old block pushed up by 1
        assert board.grid[Vector2D(1, 3)] == Mino.T

    def test_garbage_removes_top_row(self) -> None:
        board = Board(num_cols=4, num_rows=6)
        _fill_row(board, 5, Mino.I)  # top row

        garbage = [Mino.GARBAGE] * 4
        board.insert_garbage(garbage)

        # top row was removed — row 5 is now old row 4
        for x in range(4):
            assert board.grid[Vector2D(x, 5)] == Mino.EMPTY
