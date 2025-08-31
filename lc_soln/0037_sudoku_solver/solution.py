from typing import List

DIM = 9
SDIM = 3
NUMS = "123456789"


class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """

        def box_idx(i, j):
            return (i // SDIM) * SDIM + j // SDIM

        rows = [set() for _ in range(DIM)]
        cols = [set() for _ in range(DIM)]
        boxs = [set() for _ in range(DIM)]
        blanks = []

        for i in range(DIM):
            for j in range(DIM):
                c = board[i][j]
                if c == ".":
                    blanks.append((i, j))
                else:
                    rows[i].add(c)
                    cols[j].add(c)
                    boxs[box_idx(i, j)].add(c)

        def solve(idx):
            if idx == len(blanks):
                return True
            i, j = blanks[idx]
            k = box_idx(i, j)
            for c in NUMS:
                if c not in rows[i] and c not in cols[j] and c not in boxs[k]:
                    board[i][j] = c
                    rows[i].add(c)
                    cols[j].add(c)
                    boxs[k].add(c)
                    if solve(idx + 1):
                        return True
                    board[i][j] = "."
                    rows[i].remove(c)
                    cols[j].remove(c)
                    boxs[k].remove(c)
            return False

        solve(0)
