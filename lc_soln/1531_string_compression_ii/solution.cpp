#include <algorithm>
#include <climits>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int getLengthOfOptimalCompression(string s, int k) {
        int n = s.size();
        vector<vector<int>> dp(n + 1, vector<int>(k + 1, INT_MAX));
        for (int j = 0; j < k + 1; ++j) {
            dp[0][j] = 0;
        }

        for (int i = 1; i <= n; ++i) {
            for (int j = 0; j <= k; ++j) {
                if (j > 0) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                int removed = 0;
                int count = 0;
                for (int p = i; p > 0; --p) {
                    if (s[p - 1] == s[i - 1]) {
                        ++count;
                    } else {
                        ++removed;
                        if (removed > j) {
                            break;
                        }
                    }
                    dp[i][j] = min(dp[i][j], dp[p - 1][j - removed] + getLen(count));
                }
            }
        }
        return dp[n][k];
    }

    int getLen(const int count) const {
        if (count <= 1) {
            return count;
        }
        if (count < 10) {
            return 2;
        }
        if (count < 100) {
            return 3;
        }
        return 4;
    }
};

