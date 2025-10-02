from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        start = 0
        curr = 0
        n = len(nums)
        result = n + 1
        for i, num in enumerate(nums):
            curr += num
            while curr >= target:
                result = min(result, i - start + 1)
                curr -= nums[start]
                start += 1
        return 0 if result == n + 1 else result
