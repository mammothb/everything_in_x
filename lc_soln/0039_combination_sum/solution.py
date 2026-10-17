class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums = sorted(nums)
        result = []
        buf = []
        self.dfs(nums, 0, len(nums), 0, buf, result, target)
        return result

    def dfs(self, nums, start, stop, curr, buf, result, target):
        if curr == target:
            result.append(buf.copy())
            return
        if start >= stop:
            return
        for i in range(start, stop):
            curr += nums[i]
            if curr > target:
                return
            buf.append(nums[i])
            self.dfs(nums, i, stop, curr, buf, result, target)
            buf.pop()
            curr -= nums[i]


def main(): ...


if __name__ == "__main__":
    main()
