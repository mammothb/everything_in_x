#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    bool isPalindrome(string s) {
        int l = 0;
        int r = s.size() - 1;
        while (l < r) {
            if (isalnum(s[l]) == 0) {
                ++l;
                continue;
            }
            if (isalnum(s[r]) == 0) {
                --r;
                continue;
            }
            if (tolower(s[l]) != tolower(s[r])) {
                return false;
            }
            ++l;
            --r;
        }
        return true;
    }
};

int main() {
    Solution solution;
    cout << solution.isPalindrome("A man, a plan, a canal: Panama") << "\n";
    return 0;
}
