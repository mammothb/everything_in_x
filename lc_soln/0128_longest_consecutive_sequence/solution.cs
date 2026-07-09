public class Solution {
    public int LongestConsecutive(int[] nums) {
        var seen = new HashSet<int>(nums);
        int result = 0;
        foreach (int num in seen) {
            if (!seen.Contains(num - 1)) {
                int count = 1;
                while (seen.Contains(num + count)) {
                    count++;
                }
                result = Math.Max(result, count);
            }
        }
        return result;
    }
}

