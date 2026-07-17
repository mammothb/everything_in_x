class Solution {
public:
    string minWindow(string s, string t) {
        std::unordered_map<char, int> counter;
        for (char c : t) {
            counter[c]++;
        }
        int need = counter.size();
        int start = 0;
        int best = INT_MAX;
        int best_start = 0;
        for (int i = 0; i < s.size(); ++i) {
            if (counter.contains(s[i])) {
                counter[s[i]]--;
                if (counter[s[i]] == 0) {
                    need--;
                }
            }
            while (start <= i && (!counter.contains(s[start]) || counter[s[start]] < 0)) {
                if (counter.contains(s[start])) {
                    counter[s[start]]++;
                }
                start++;
            }
            if (need == 0 && i - start + 1 < best) {
                best = i - start + 1;
                best_start = start;
            }
        }
        return best == INT_MAX ? "" : s.substr(best_start, best);
    }
};

int main() {
    return 0;
}
