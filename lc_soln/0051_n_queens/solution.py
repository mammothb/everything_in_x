class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result = []
        curr = [["."] * n for _ in range(n)]
        self.dfs(0, n, curr, result)
        return result

    def dfs(self, j, n, curr, result):
        if j == n:
            result.append(["".join(curr[i]) for i in range(n)])
            return
        for i in range(n):
            if not self.is_valid(j, n, i, curr):
                continue
            curr[i][j] = "Q"
            self.dfs(j + 1, n, curr, result)
            curr[i][j] = "."

    def is_valid(self, col, n, row, curr):
        for j in range(col):
            if curr[row][j] != ".":
                return False
        i = row - 1
        j = col - 1
        while i >= 0 and j >= 0:
            if curr[i][j] != ".":
                return False
            i -= 1
            j -= 1

        i = row + 1
        j = col - 1
        while i < n and j >= 0:
            if curr[i][j] != ".":
                return False
            i += 1
            j -= 1

        return True


def main(): ...


if __name__ == "__main__":
    main()
