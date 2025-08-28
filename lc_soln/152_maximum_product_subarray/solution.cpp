#include <climits>
#include <ranges>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int result = INT_MIN;
        int curr = 1;
        for (const int num : nums) {
            curr *= num;
            result = max(result, curr);
            if (curr == 0) {
                curr = 1;
            }
        }
        curr = 1;
        for (const int num : nums | std::views::reverse) {
            curr *= num;
            result = max(result, curr);
            if (curr == 0) {
                curr = 1;
            }
        }
        return result;
    }
};

