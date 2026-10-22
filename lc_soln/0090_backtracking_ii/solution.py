class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums = sorted(nums)
        result = []
        curr = []
        self.dfs(nums, 0, len(nums), curr, result)
        return result

    def dfs(self, nums, start, stop, curr, result):
        result.append(curr.copy())
        for i in range(start, stop):
            if i > start and nums[i] == nums[i - 1]:
                continue
            curr.append(nums[i])
            self.dfs(nums, i + 1, stop, curr, result)
            curr.pop()


def main(): ...


if __name__ == "__main__":
    main()
