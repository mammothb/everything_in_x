public class Solution
{
    public List<List<int>> Subsets(int[] nums)
    {
        List<List<int>> result = [];
        List<int> curr = [];
        Dfs(nums, 0, nums.Length, curr, result);
        return result;
    }

    void Dfs(int[] nums, int start, int stop, List<int> curr, List<List<int>> result)
    {
        result.Add(new List<int>(curr));
        if (start >= stop)
        {
            return;
        }
        for (int i = start; i < stop; i++)
        {
            curr.Add(nums[i]);
            Dfs(nums, i + 1, stop, curr, result);
            curr.RemoveAt(curr.Count - 1);
        }
    }
}
