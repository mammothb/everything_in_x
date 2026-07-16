from collections import Counter


class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        counter = Counter(s1)
        curr = counter.copy()
        start = 0
        for i, c in enumerate(s2):
            if c not in counter:
                curr = counter.copy()
                continue
            curr[c] -= 1
            while curr[c] < 0:
                curr[s2[start]] += 1
                start += 1
            if all(val == 0 for val in curr.values()):
                return True
        return False


def main(): ...


if __name__ == "__main__":
    main()
