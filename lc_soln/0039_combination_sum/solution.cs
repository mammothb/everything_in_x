public class Solution
{
    public List<List<int>> CombinationSum(int[] nums, int target)
    {
        Array.Sort(nums);
        List<List<int>> result = [];
        List<int> buf = [];
        Dfs(nums, 0, nums.Length, 0, buf, result, target);
        return result;
    }

    void Dfs(
        int[] nums,
        int start,
        int stop,
        int curr,
        List<int> buf,
        List<List<int>> result,
        int target
    )
    {
        if (curr == target)
        {
            result.Add([.. buf]);
            return;
        }
        if (start >= stop)
        {
            return;
        }
        for (int i = start; i < stop; i++)
        {
            curr += nums[i];
            if (curr > target)
            {
                return;
            }
            buf.Add(nums[i]);
            Dfs(nums, i, stop, curr, buf, result, target);
            buf.RemoveAt(buf.Count - 1);
            curr -= nums[i];
        }
    }
}
