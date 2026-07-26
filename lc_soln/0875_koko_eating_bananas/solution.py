import math
from typing import List


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        l = 1
        r = max(piles)
        result = r
        while l <= r:
            mid = l + (r - l) // 2
            count = sum(math.ceil(pile / mid) for pile in piles)
            if count <= h:
                result = min(result, mid)
                r = mid - 1
            else:
                l = mid + 1
        return result


def main(): ...


if __name__ == "__main__":
    main()
