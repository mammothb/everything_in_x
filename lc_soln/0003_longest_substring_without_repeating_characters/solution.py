class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        result = 0
        start = 0
        counter = {}
        for i, c in enumerate(s):
            while counter.get(c, 0) > 0:
                counter[s[start]] -= 1
                start += 1
            counter[c] = counter.setdefault(c, 0) + 1
            result = max(result, i - start + 1)
        return result


def main(): ...


if __name__ == "__main__":
    main()
