#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> seen;
        vector<int> result;
        for (int i = 0; i < nums.size(); ++i) {
            if (seen.contains(target - nums[i])) {
                return {i, seen[target - nums[i]]};
            }
            seen[nums[i]] = i;
        }
        return {};
    }
};

int main() {
    Solution solution;
    vector<int> nums = {2, 7, 11, 15};
    auto result = solution.twoSum(nums, 9);
    std::cout << result[0] << " " << result[1] << "\n";
    return 0;
}
