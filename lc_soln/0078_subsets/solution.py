class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result = [[]]
        n = len(nums)
        for i in range(n):
            curr = []
            self.dfs(nums, i, len(nums), curr, result)
        return result

    def dfs(self, nums, start, stop, curr, result):
        if start == stop:
            return
        curr.append(nums[start])
        result.append(curr.copy())
        for i in range(start + 1, stop):
            self.dfs(nums, i, stop, curr, result)
        curr.pop()


def main(): ...


if __name__ == "__main__":
    main()
