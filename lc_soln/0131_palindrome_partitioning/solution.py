class Solution:
    def partition(self, s: str) -> List[List[str]]:
        result = []
        curr = []
        self.dfs(s, curr, result)
        return result

    def dfs(self, s, curr, result):
        if not s:
            result.append(curr.copy())
            return
        for i in range(1, len(s) + 1):
            word = s[:i]
            if self.is_palindrome(word):
                curr.append(word)
                self.dfs(s[i:], curr, result)
                curr.pop()

    def is_palindrome(self, word):
        l = 0
        r = len(word) - 1
        while l < r:
            if word[l] != word[r]:
                return False
            l += 1
            r -= 1
        return True


def main(): ...


if __name__ == "__main__":
    main()
