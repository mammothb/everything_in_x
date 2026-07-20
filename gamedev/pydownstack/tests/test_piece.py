from pydownstack.domain.mino import Mino
from pydownstack.domain.rotation import Rotation
from pydownstack.domain.vector import Vector2D
from pydownstack.game.board import Board
from pydownstack.game.config import GuidelineConfig
from pydownstack.game.piece import get_cells, try_rotation

# ── get_cells ────────────────────────────────────────────────────────


class TestGetCells:
    def test_origin_shifts_all_cells(self, guideline_config: GuidelineConfig) -> None:
        piece = guideline_config.pieces[Mino.O]
        cells = get_cells(piece, rot=0, origin=Vector2D(5, 10))
        # O rot 0: [[1,1], [2,1], [1,2], [2,2]]
        assert cells == [
            Vector2D(6, 11),
            Vector2D(7, 11),
            Vector2D(6, 12),
            Vector2D(7, 12),
        ]

    def test_different_rotations_produce_different_cells(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.T]
        cells_0 = get_cells(piece, rot=0, origin=Vector2D(3, 18))
        cells_R = get_cells(piece, rot=1, origin=Vector2D(3, 18))
        assert cells_0 != cells_R

    def test_zero_origin_returns_raw_coords(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.T]
        cells = get_cells(piece, rot=0, origin=Vector2D(0, 0))
        assert cells == piece.coords[0]

    def test_four_cells_per_piece(self, guideline_config: GuidelineConfig) -> None:
        for mino in guideline_config.pieces:
            piece = guideline_config.pieces[mino]
            for rot in range(4):
                cells = get_cells(piece, rot, origin=Vector2D(0, 0))
                assert len(cells) == 4


# ── try_rotation ─────────────────────────────────────────────────────


class TestTryRotation:
    def test_empty_board_succeeds_with_first_kick(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CW, board=board
        )
        assert result == (1, Vector2D(3, 18))

    def test_ccw_from_spawn(self, guideline_config: GuidelineConfig) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CCW, board=board
        )
        assert result == (3, Vector2D(3, 18))

    def test_cw_wraps_from_3_to_0(self, guideline_config: GuidelineConfig) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        result = try_rotation(
            piece, rot=3, origin=Vector2D(3, 18), rotation=Rotation.CW, board=board
        )
        assert result is not None
        assert result[0] == 0

    def test_ccw_wraps_from_0_to_3(self, guideline_config: GuidelineConfig) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CCW, board=board
        )
        assert result is not None
        assert result[0] == 3

    def test_all_kicks_fail_returns_none(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        for y in range(25):
            for x in range(10):
                board.grid[Vector2D(x, y)] = Mino.GARBAGE
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CW, board=board
        )
        assert result is None

    def test_second_kick_used_when_first_blocked(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.T]
        board = Board(num_cols=10, num_rows=25)
        # T rot 0→1 CW kicks: [[0,0], [-1,0], [-1,1], [0,-2], [-1,-2]]
        # First kick [0,0] places rot=1 cells at origin (3,18):
        #   T rot 1 coords: [[1,2], [1,1], [1,0], [2,1]]
        #   world: [(4,20), (4,19), (4,18), (5,19)]
        # Block (4,18) to force second kick
        board.grid[Vector2D(4, 18)] = Mino.GARBAGE
        # Second kick [-1,0] shifts origin to (2,18):
        #   world: [(3,20), (3,19), (3,18), (4,19)]
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CW, board=board
        )
        assert result == (1, Vector2D(2, 18))

    def test_o_piece_returns_none(self, guideline_config: GuidelineConfig) -> None:
        piece = guideline_config.pieces[Mino.O]
        board = Board(num_cols=10, num_rows=25)
        result = try_rotation(
            piece, rot=0, origin=Vector2D(4, 18), rotation=Rotation.CW, board=board
        )
        assert result is None

    def test_i_piece_kicks_with_distinct_offsets(
        self, guideline_config: GuidelineConfig
    ) -> None:
        piece = guideline_config.pieces[Mino.I]
        board = Board(num_cols=10, num_rows=25)
        # I rot 0→1 CW kicks: [[0,0], [-2,0], [1,0], [-2,-1], [1,2]]
        # First kick [0,0]: rot=1, origin (3,18)
        #   I rot 1 coords: [[2,3], [2,2], [2,1], [2,0]]
        #   world: [(5,21), (5,20), (5,19), (5,18)]
        # Block (5,19) to force second kick
        board.grid[Vector2D(5, 19)] = Mino.GARBAGE
        # Second kick [-2,0]: origin (1,18)
        #   world: [(3,21), (3,20), (3,19), (3,18)]
        result = try_rotation(
            piece, rot=0, origin=Vector2D(3, 18), rotation=Rotation.CW, board=board
        )
        assert result == (1, Vector2D(1, 18))
