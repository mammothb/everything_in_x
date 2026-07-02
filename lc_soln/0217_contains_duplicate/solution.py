from typing import List


class Solution:
    def has_duplicate(self, nums: List[int]) -> bool:
        seen: set[int] = set()
        for num in nums:
            if num in nums:
                return True
            seen.add(num)
        return False


def main():
    print(Solution().has_duplicate(nums=[1, 2, 3, 3]))


if __name__ == "__main__":
    main()
