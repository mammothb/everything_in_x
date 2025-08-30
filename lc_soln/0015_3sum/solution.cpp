#include <algorithm>
#include <cstddef>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        ranges::sort(nums);
        vector<vector<int>> result;
        const size_t n = nums.size();
        for (size_t i = 0; i < n; ++i) {
            if (nums[i] > 0) {
                break;
            }
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            size_t l = i + 1;
            size_t r = n - 1;
            while (l < r) {
                const int total = nums[i] + nums[l] + nums[r];
                if (total > 0) {
                    --r;
                } else if (total < 0) {
                    ++l;
                } else {
                    result.push_back({nums[i], nums[l], nums[r]});
                    do {
                        ++l;
                    } while (l < r && nums[l] == nums[l - 1]);
                    --r;
                }
            }
        }
        return result;
    }
};

