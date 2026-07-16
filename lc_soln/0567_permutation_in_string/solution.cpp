class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        std::vector<int> counter(26, 0);
        for (char c : s1) {
            counter[c - 'a']++;
        }
        std::vector<int> curr(26, 0);
        int start = 0;
        for (int i = 0; i < s2.size(); ++i) {
            if (counter[s2[i] - 'a'] == 0) {
                std::fill(curr.begin(), curr.end(), 0);
                start = i + 1;
                continue;
            }
            curr[s2[i] - 'a']++;
            while (curr[s2[i] - 'a'] > counter[s2[i] - 'a']) {
                curr[s2[start] - 'a']--;
                start++;
            }
            if (i - start + 1 == s1.size()) {
                return true;
            }
        }
        return false;
    }
};

int main() {
    return 0;
}
