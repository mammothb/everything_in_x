from typing import List


class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        count = 1
        last = nums[0]
        for i in range(1, len(nums)):
            num = nums[i]
            if num == last:
                count += 1
            else:
                count -= 1
                if count == 0:
                    last = num
                    count = 1
        return last
