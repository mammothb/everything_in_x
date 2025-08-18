#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int result = 0;
        for (int i = 1; i < prices.size(); ++i) {
            if (int profit = prices[i] - prices[i - 1]; profit > 0) {
                result += profit;
            }
        }
        return result;
    }
};

int main() {
    Solution solution;
    vector<int> prices = {7, 1, 5, 3, 6, 4};
    cout << solution.maxProfit(prices) << "\n";
}
