class Solution {
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        int n = nums.size();
        int result = n + 1;
        int curr = 0;
        int start = 0;
        for (int i = 0; i < n; ++i) {
            curr += nums[i];
            while (curr >= target) {
                result = min(result, i - start + 1);
                curr -= nums[start];
                ++start;
            }
        }
        return result == n + 1 ? 0 : result;
    }
};

