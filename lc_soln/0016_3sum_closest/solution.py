from typing import List


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums = sorted(nums)
        result = 0
        min_diff = float("inf")
        n = len(nums)
        for i in range(n):
            l = i + 1
            r = n - 1
            while l < r:
                total = nums[i] + nums[l] + nums[r]
                if total == target:
                    return total
                diff = abs(total - target)
                if diff < min_diff:
                    min_diff = diff
                    result = total
                if total > target:
                    r -= 1
                else:
                    l += 1
        return result
