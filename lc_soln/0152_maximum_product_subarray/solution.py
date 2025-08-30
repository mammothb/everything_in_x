from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        result = float("-inf")

        curr = 1
        for num in nums:
            curr *= num
            result = max(result, curr)
            if curr == 0:
                curr = 1

        curr = 1
        for num in reversed(nums):
            curr *= num
            result = max(result, curr)
            if curr == 0:
                curr = 1

        return result
