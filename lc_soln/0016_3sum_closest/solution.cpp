#include <algorithm>
#include <climits>
#include <vector>

using namespace std;

class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        ranges::sort(nums);
        int result = 0;
        int min_diff = INT_MAX;
        for (int i = 0; i < nums.size() - 2; ++i) {
            int l = i + 1;
            int r = nums.size() - 1;
            while (l < r) {
                int total = nums[i] + nums[l] + nums[r];
                if (total == target) {
                    return total;
                }
                if (int diff = abs(total - target); diff < min_diff) {
                    min_diff = diff;
                    result = total;
                }
                if (total > target) {
                    r--;
                } else {
                    l++;
                }
            }
        }
        return result;
    }
};

