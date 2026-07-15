from collections import defaultdict


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        result = 0
        start = 0
        high = 0
        counter = defaultdict(int)
        for i, c in enumerate(s):
            counter[c] += 1
            high = max(high, counter[c])
            while i - start + 1 - high > k:
                counter[s[start]] -= 1
                start += 1
            result = max(result, i - start + 1)
        return result


def main(): ...


if __name__ == "__main__":
    main()
