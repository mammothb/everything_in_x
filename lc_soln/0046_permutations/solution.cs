public class Solution
{
    public List<List<int>> Permute(int[] nums)
    {
        List<List<int>> result = [];
        Dfs(nums, 0, nums.Length, result);
        return result;
    }

    void Dfs(int[] nums, int start, int stop, List<List<int>> result)
    {
        if (start == stop)
        {
            result.Add(nums.ToList());
            return;
        }
        for (int i = start; i < stop; i++)
        {
            (nums[i], nums[start]) = (nums[start], nums[i]);
            Dfs(nums, start + 1, stop, result);
            (nums[i], nums[start]) = (nums[start], nums[i]);
        }
    }
}
