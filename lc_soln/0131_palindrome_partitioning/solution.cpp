class Solution {
   public:
    vector<vector<string>> partition(string s) {
        std::vector<std::vector<std::string>> result;
        std::vector<std::string> curr;
        dfs(0, s, curr, result);
        return result;
    }

    void dfs(int i, const std::string& s, std::vector<std::string>& curr,
             std::vector<std::vector<std::string>>& result) {
        if (i >= s.size()) {
            result.push_back(curr);
            return;
        }
        for (int j = i; j < s.size(); j++) {
            if (isPalindrome(s, i, j)) {
                curr.push_back(s.substr(i, j - i + 1));
                dfs(j + 1, s, curr, result);
                curr.pop_back();
            }
        }
    }

    bool isPalindrome(const std::string& word, int l, int r) {
        while (l < r) {
            if (word[l] != word[r]) {
                return false;
            }
            l++;
            r--;
        }
        return true;
    }
};

int main() {
    return 0;
}
