public class Solution
{
    public int[] MaxSlidingWindow(int[] nums, int k)
    {
        var q = new LinkedList<int>();
        var result = new List<int>();
        int start = 0;
        for (int i = 0; i < nums.Length; ++i)
        {
            while (q.Count > 0 && nums[q.Last.Value] < nums[i])
            {
                q.RemoveLast();
            }
            q.AddLast(i);
            if (q.First.Value < start)
            {
                q.RemoveFirst();
            }
            if (i - start + 1 >= k)
            {
                result.Add(nums[q.First.Value]);
                start++;
            }
        }
        return result.ToArray();
    }
}
