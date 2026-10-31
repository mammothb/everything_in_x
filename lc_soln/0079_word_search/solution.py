class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        nr = len(board)
        nc = len(board[0])
        for i in range(nr):
            for j in range(nc):
                if self.dfs(board, i, j, 0, word):
                    return True
        return False

    def dfs(self, board, row, col, pos, word):
        if pos == len(word):
            return True
        nr = len(board)
        nc = len(board[0])
        if not (0 <= row < nr and 0 <= col < nc):
            return False

        c = board[row][col]
        if c == "." or c != word[pos]:
            return False

        board[row][col] = "."
        for i, j in ((row + 1, col), (row - 1, col), (row, col + 1), (row, col - 1)):
            if self.dfs(board, i, j, pos + 1, word):
                return True

        board[row][col] = c
        return FAlse


def main(): ...


if __name__ == "__main__":
    main()
