public class Solution {
    public int[] TwoSum(int[] numbers, int target) {
        int l = 0;
        int r = numbers.Length - 1;
        while (true) {
            int diff = numbers[l] + numbers[r] - target;
            if (diff < 0) {
                l++;
            } else if (diff > 0) {
                r--;
            } else {
                return [l + 1, r + 1];
            }
        }
        return [];
    }
}

