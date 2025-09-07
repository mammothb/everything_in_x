from typing import List


class Solution:
    def maximumUniqueSubarray(self, nums: List[int]) -> int:
        seen = set()
        curr = 0
        result = 0
        start = 0
        for i, num in enumerate(nums):
            while num in seen:
                seen.remove(nums[start])
                curr -= nums[start]
                start += 1
            seen.add(num)
            curr += num
            result = max(result, curr)
        return result
