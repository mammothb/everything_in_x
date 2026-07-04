from collections import Counter
from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        counter = Counter(nums)
        num_and_freq = sorted(
            [(num, freq) for num, freq in counter.items()], key=lambda elem: -elem[1]
        )
        return [num_and_freq[i][0] for i in range(k)]


def main(): ...


if __name__ == "__main__":
    main()
