class Solution {
public:
    int maximumUniqueSubarray(vector<int>& nums) {
        unordered_set<int> seen;
        int curr = 0;
        int result = 0;
        int start = 0;
        for (int i = 0; i < nums.size(); ++i) {
            int num = nums[i];
            while (seen.contains(num)) {
                seen.erase(nums[start]);
                curr -= nums[start];
                ++start;
            }
            seen.insert(num);
            curr += num;
            result = max(result, curr);
        }
        return result;
    }
};

