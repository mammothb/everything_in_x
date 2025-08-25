#include <string>

using namespace std;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        int size = 0;
        int i = 0;
        while (true) {
            if (i >= strs[0].size()) {
                return strs[0].substr(0, size);
            }
            char c = strs[0][i];
            for (const string& s : strs) {
                if (i >= s.size()) {
                    return strs[0].substr(0, size);
                }
                if (s[i] != c) {
                    return strs[0].substr(0, size);
                }
            }
            ++size;
            ++i;
        }
        return strs[0].substr(0, size);
    }
};

