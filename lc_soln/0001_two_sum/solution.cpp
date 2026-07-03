#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> seen;
        for (int i = 0; i < nums.size(); ++i) {
            int num = nums[i];
            if (seen.contains(num)) {
                return {i, seen[num]};
            }
            seen[target - num] = i;
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
