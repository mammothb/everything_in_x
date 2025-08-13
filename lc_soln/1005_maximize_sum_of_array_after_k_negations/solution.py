import heapq
from typing import List


class Solution:
    def largestSumAfterKNegations(self, nums: List[int], k: int) -> int:
        total = sum(nums)
        heapq.heapify(nums)

        while k > 0:
            num = heapq.heappop(nums)
            num = -num
            heapq.heappush(nums, num)
            if num == 0:
                break
            total += 2 * num
            k -= 1
        return total
