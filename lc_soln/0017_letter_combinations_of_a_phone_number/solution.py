class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        digit_to_letter = {
            "2": "abc",
            "3": "def",
            "4": "ghi",
            "5": "jkl",
            "6": "mno",
            "7": "pqrs",
            "8": "tuv",
            "9": "wxyz",
        }
        result = []
        self.dfs(digit_to_letter, digits, 0, "", result)
        return result

    def dfs(self, digit_to_letter, digits, pos, curr, result):
        if pos == len(digits):
            if curr:
                result.append(curr)
            return
        for c in digit_to_letter[digits[pos]]:
            self.dfs(digit_to_letter, digits, pos + 1, curr + c, result)


def main(): ...


if __name__ == "__main__":
    main()
