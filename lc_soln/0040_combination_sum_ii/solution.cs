public class Solution
{
    public List<List<int>> CombinationSum2(int[] candidates, int target)
    {
        Array.Sort(candidates);
        List<List<int>> result = [];
        List<int> curr = [];
        Dfs(candidates, 0, candidates.Length, target, curr, result);
        return result;
    }

    void Dfs(int[] nums, int start, int stop, int target, List<int> curr, List<List<int>> result)
    {
        if (target == 0)
        {
            result.Add([.. curr]);
            return;
        }
        if (start >= stop)
        {
            return;
        }
        for (int i = start; i < stop; i++)
        {
            if (i > start && nums[i] == nums[i - 1])
            {
                continue;
            }
            target -= nums[i];
            if (target < 0)
            {
                return;
            }
            curr.Add(nums[i]);
            Dfs(nums, i + 1, stop, target, curr, result);
            curr.RemoveAt(curr.Count - 1);
            target += nums[i];
        }
    }
}
