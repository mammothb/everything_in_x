#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxArea(vector<int>& height) {
        int l = 0;
        int r = height.size() - 1;
        int result = 0;
        while (l < r) {
            result = max(result, min(height[l], height[r]) * (r - l));
            if (height[l] < height[r]) {
                ++l;
            } else {
                --r;
            }
        }
        return result;
    }
};

int main() {
    Solution solution;
    vector<int> height= {1, 8, 6, 2, 5, 4, 8, 3, 7};
    cout << solution.maxArea(height) << "\n";
    return 0;
}
