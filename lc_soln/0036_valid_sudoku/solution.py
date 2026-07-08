from typing import List

SIZE = 9
BSIZE = 3


def blk_idx(row: int, col: int) -> int:
    return (row // BSIZE) * BSIZE + (col // BSIZE)


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rows = [0] * SIZE
        cols = [0] * SIZE
        blks = [0] * SIZE
        for i in range(SIZE):
            for j in range(SIZE):
                if board[i][j] == ".":
                    continue
                offset = 1 << int(board[i][j])
                if (
                    (rows[i] & offset)
                    or (cols[j] & offset)
                    or (blks[blk_idx(i, j)] & offset)
                ):
                    return False
                rows[i] |= offset
                cols[j] |= offset
                blks[blk_idx(i, j)] |= offset

        return True


def main(): ...


if __name__ == "__main__":
    main()
