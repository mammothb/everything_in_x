class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        vector<vector<int>> result;
        ranges::sort(nums);
        for (int i = 0; i < nums.size(); ++i) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            for (int j = i + 1; j < nums.size(); ++j) {
                if (j > i + 1 && nums[j] == nums[j - 1]) {
                    continue;
                }
                long long curr = nums[i] + nums[j];
                int l = j + 1;
                int r = nums.size() - 1;
                while (l < r) {
                    long long total = curr + nums[l] + nums[r];
                    if (total > target) {
                        --r;
                    } else if (total < target) {
                        ++l;
                    } else {
                        result.push_back({nums[i], nums[j], nums[l], nums[r]});
                        ++l;
                        while (l < r && nums[l] == nums[l - 1]) {
                            ++l;
                        }
                        --r;
                        while (l < r && nums[r] == nums[r + 1]) {
                            --r;
                        }
                    }
                }
            }
        }
        return result;
    }
};
