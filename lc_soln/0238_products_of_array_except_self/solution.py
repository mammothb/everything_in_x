from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        fwd = [1] * (n + 2)
        bwd = [1] * (n + 2)
        for i in range(n):
            fwd[i + 1] = fwd[i] * nums[i]
            bwd[n - i] = bwd[n - i + 1] * nums[n - 1 - i]
        result = []
        for i in range(n):
            result.append(fwd[i] * bwd[i + 2])

        return result


def main(): ...


if __name__ == "__main__":
    main()
