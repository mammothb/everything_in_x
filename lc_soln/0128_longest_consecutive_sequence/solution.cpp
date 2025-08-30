#include <algorithm>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_set<int> seen(nums.begin(), nums.end());
        int result = 0;
        for (const auto& num : seen) {
            if (!seen.contains(num - 1)) {
                int count = 1;
                while (seen.contains(num + count)) {
                    ++count;
                }
                result = max(result, count);
            }
        }
        return result;
    }
};

int main() {
    Solution solution;
    vector<int> nums = {100, 4, 200, 1, 3, 2};
    cout << solution.longestConsecutive(nums) << "\n";
    return 0;
}
