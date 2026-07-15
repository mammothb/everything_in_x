class Solution {
public:
    int characterReplacement(string s, int k) {
        int result = 0;
        int start = 0;
        int high = 0;
        std::vector<int> counter(26, 0);
        for (int i = 0; i < s.size(); ++i) {
            counter[s[i] - 'A']++;
            high = std::max(high, counter[s[i] - 'A']);
            while (i - start + 1 - high > k) {
                counter[s[start] - 'A']--;
                start++;
            }
            result = std::max(result, i - start + 1);
        }
        return result;
    }
};

int main() {
    return 0;
}
