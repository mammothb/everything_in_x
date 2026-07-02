class Solution {
public:
    bool isAnagram(string s, string t) {
        std::vector<int> counter_s(26, 0);
        std::vector<int> counter_t(26, 0);
        for (char c : s) {
            counter_s[c - 'a']++;
        }
        for (char c : t) {
            counter_t[c - 'a']++;
        }
        for (int i = 0; i < 26; ++i) {
            if (counter_s[i] != counter_t[i]) {
                return false;
            }
        }
        return true;
    }
};

int main() {
    return 0;
}
