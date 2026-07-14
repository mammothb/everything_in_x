class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int result = 0;
        int start = 0;
        std::unordered_map<char, int> counter;
        for (int i = 0; i < s.size(); ++i) {
            while (counter[s[i]] > 0) {
                counter[s[start]]--;
                start++;
            }
            counter[s[i]]++;
            result = std::max(result, i - start + 1);
        }
        return result;
    }
};

int main() {
    return 0;
}
