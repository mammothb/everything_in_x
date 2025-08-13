#include <functional>
#include <iostream>
#include <numeric>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    int largestSumAfterKNegations(vector<int>& nums, int k) {
        int total = reduce(nums.begin(), nums.end());
        priority_queue<int, vector<int>, greater<int>> pq(nums.begin(), nums.end());
        while (k > 0) {
            int num = pq.top();
            if (num == 0) {
                break;
            }
            pq.pop();
            pq.push(-num);
            total -= 2 * num;
            --k;
        }
        return total;
    }
};

int main() {
    Solution solution;
    vector<int> nums = {4, 2, 3};
    cout << solution.largestSumAfterKNegations(nums, 1) << "\n";
    return 0;
}
