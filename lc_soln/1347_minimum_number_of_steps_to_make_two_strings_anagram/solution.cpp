#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int minSteps(string s, string t) {
        vector<int> counter_s(26, 0);
        vector<int> counter_t(26, 0);
        for (int i = 0; i < s.size(); ++i) {
            counter_s[s[i] - 'a']++;
            counter_t[t[i] - 'a']++;
        }

        int result = 0;
        for (int i = 0; i < 26; ++i) {
            if (int diff = counter_s[i] - counter_t[i]; diff > 0) {
                result += diff;
            }
        }
        return result;
    }
};
