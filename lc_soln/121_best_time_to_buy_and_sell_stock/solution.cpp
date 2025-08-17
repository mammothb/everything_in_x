#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int result = 0;
        int buy = prices[0];
        for (int i = 1; i < prices.size(); ++i) {
            buy = min(buy, prices[i]);
            result = max(result, prices[i] - buy);
        }
        return result;
    }
};

int main() {
    Solution solution;
    vector<int> prices = {7, 1, 5, 3, 6, 4};
    cout << solution.maxProfit(prices) << "\n";
    return 0;
}
