from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums = sorted(nums)
        n = len(nums)
        result = []
        for i, num in enumerate(nums):
            if num > 0:
                break
            if i > 0 and num == nums[i - 1]:
                continue
            l = i + 1
            r = n - 1
            while l < r:
                total = num + nums[l] + nums[r]
                if total > 0:
                    r -= 1
                elif total < 0:
                    l += 1
                else:
                    result.append([num, nums[l], nums[r]])
                    left = nums[l]
                    while l < r and nums[l] == left:
                        l += 1
                    r -= 1
        return result
