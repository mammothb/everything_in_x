#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
public:
    int shipWithinDays(vector<int>& weights, int days) {
        int l = ranges::max(weights);
        int r = reduce(weights.begin(), weights.end());
        int result = 0;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (timeTaken(weights, mid) <= days) {
                result = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return result;
    }

    int timeTaken(const vector<int>& weights, int capacity) {
        int result = 1;
        int curr = 0;
        for (const auto& w : weights) {
            curr += w;
            if (curr > capacity) {
                curr = w;
                ++result;
            }
        }
        return result;
    }
};

int main() {
    Solution solution;
    vector<int> weights = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    cout << solution.shipWithinDays(weights, 5) << "\n";
    return 0;
}
