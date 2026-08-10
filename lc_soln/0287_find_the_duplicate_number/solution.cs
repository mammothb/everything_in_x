public class Solution
{
    public int FindDuplicate(int[] nums)
    {
        int fast = nums[0];
        int slow = nums[0];
        int result = -1;
        while (true)
        {
            fast = nums[nums[fast]];
            slow = nums[slow];
            if (fast == slow)
            {
                int slow2 = nums[0];
                while (slow != slow2)
                {
                    slow = nums[slow];
                    slow2 = nums[slow2];
                }
                result = slow;
                break;
            }
        }
        return result;
    }
}
