#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> productQueries(int n, vector<vector<int>>& queries) {
        vector<int> powers;
        int p = 1;
        while (p <= n) {
            if ((p & n) != 0) {
                powers.push_back(p);
            }
            p <<= 1;
        }
        vector<int> result;
        result.reserve(queries.size());
        for (const auto& q : queries) {
            long long val = 1;
            for (int i = q[0]; i <= q[1]; ++i) {
                val = (val * powers[i]) % 1000000007;
            }
            result.push_back(val);
        }
        return result;
    }
};

void printResult(vector<int>& result) {
    for (const auto& val : result) {
        std::cout << val << " ";
    }
    std::cout << "\n";
}

int main() {
    Solution solution;

    vector<vector<int>> queries1 = {{0, 1}, {2, 2}, {0, 3}};
    auto result1 = solution.productQueries(15, queries1);
    printResult(result1);

    vector<vector<int>> queries2 = {{0, 0}};
    auto result2 = solution.productQueries(2, queries2);
    printResult(result2);

    return 0;
}
