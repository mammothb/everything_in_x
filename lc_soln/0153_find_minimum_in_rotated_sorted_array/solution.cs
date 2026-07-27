public class Solution
{
    public int FindMin(int[] nums)
    {
        if (nums[0] < nums[^1])
        {
            return nums[0];
        }
        int l = 0;
        int r = nums.Length - 1;
        while (l <= r)
        {
            int mid = l + (r - l);
            if (nums[mid] > nums[^1])
            {
                l = mid + 1;
            }
            else
            {
                r = mid - 1;
            }
        }
        return nums[l];
    }
}
