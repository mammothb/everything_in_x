from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        if nums[0] < nums[-1]:
            return nums[0]
        l = 0
        r = len(nums) - 1
        while l <= r:
            mid = l + (r - l) // 2
            if nums[mid] > nums[-1]:
                l = mid + 1
            else:
                r = mid - 1
        return nums[l]


def main(): ...


if __name__ == "__main__":
    main()
