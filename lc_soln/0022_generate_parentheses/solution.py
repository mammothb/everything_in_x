class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        result = []
        curr = []
        self.dfs(0, n, curr, result)
        return result

    def dfs(self, left, right, curr, result):
        if left == 0 and right == 0:
            result.append("".join(curr))
            return

        if left > 0:
            curr.append(")")
            self.dfs(left - 1, right, curr, result)
            curr.pop()

        if right > 0:
            curr.append("(")
            self.dfs(left + 1, right - 1, curr, result)
            curr.pop()


def main(): ...


if __name__ == "__main__":
    main()
