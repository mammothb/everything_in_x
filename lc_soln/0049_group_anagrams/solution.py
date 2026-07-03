from collections import defaultdict
from typing import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        variants: defaultdict[tuple[int, ...], list[str]] = defaultdict(list)
        for s in strs:
            counter = [0] * 26
            for c in s:
                counter[ord(c) - ord("a")] += 1
            variants[tuple(counter)].append(s)
        return list(variants.values())


def main():
    print(Solution().groupAnagrams(["act", "pots", "tops", "cat", "stop", "hat"]))


if __name__ == "__main__":
    main()
