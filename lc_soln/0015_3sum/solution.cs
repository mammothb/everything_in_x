public class Solution {
    public List<List<int>> ThreeSum(int[] nums) {
        Array.Sort(nums);
        int n = nums.Length;
        List<List<int>> result = [];
        for (int i = 0; i < n; ++i) {
            if (nums[i] > 0) {
                break;
            }
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            int l = i + 1;
            int r = n - 1;
            while (l < r) {
                int total = nums[i] + nums[l] + nums[r];
                if (total > 0) {
                    r--;
                } else if (total < 0) {
                    l++;
                } else {
                    result.Add([nums[i], nums[l], nums[r]]);
                    while (l < r) {
                        l++;
                        if (nums[l] != nums[l - 1]) {
                            break;
                        }
                    }
                    r--;
                }
            }
        }
        return result;
    }
}

