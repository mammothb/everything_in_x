class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        result = []
        self.dfs(nums, 0, len(nums), result)
        return result

    def dfs(self, nums, start, stop, result):
        if start == stop:
            result.append(nums.copy())
            return
        for i in range(start, stop):
            nums[start], nums[i] = nums[i], nums[start]
            self.dfs(nums, start + 1, stop, result)
            nums[start], nums[i] = nums[i], nums[start]


def main(): ...


if __name__ == "__main__":
    main()
