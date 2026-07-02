from collections import Counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False

        counter_s = Counter(s)
        counter_t = Counter(t)
        for c, count in counter_s.items():
            if counter_t[c] != count:
                return False
        for c, count in counter_t.items():
            if counter_s[c] != count:
                return False
        return True


def main():
    print(Solution().isAnagram("racecar", "carrace"))


if __name__ == "__main__":
    main()
