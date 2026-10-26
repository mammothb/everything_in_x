class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates = sorted(candidates)
        result = []
        curr = []
        self.dfs(candidates, 0, len(candidates), target, curr, result)
        return result

    def dfs(self, nums, start, stop, target, curr, result):
        if target == 0:
            result.append(curr.copy())
            return
        if start >= stop:
            return
        for i in range(start, stop):
            if i > start and nums[i] == nums[i - 1]:
                continue
            target -= nums[i]
            if target < 0:
                return
            curr.append(nums[i])
            self.dfs(nums, i + 1, stop, target, curr, result)
            curr.pop()
            target += nums[i]


def main(): ...


if __name__ == "__main__":
    main()
