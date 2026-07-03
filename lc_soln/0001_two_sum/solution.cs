public class Solution {
    public int[] TwoSum(int[] nums, int target) {
        var seen = new Dictionary<int, int>(nums.Length);
        for (int i = 0; i < nums.Length; ++i) {
            int num = nums[i];
            if (seen.TryGetValue(num, out int j)) {
                return [j, i];
            }
            seen.Add(target - num, i);
        }
        return [];
    }
}
