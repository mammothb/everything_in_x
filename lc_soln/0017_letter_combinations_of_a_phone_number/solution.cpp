class Solution {
   public:
    vector<string> letterCombinations(string digits) {
        std::array<std::string, 8> letters = {"abc", "def",  "ghi", "jkl",
                                              "mno", "pqrs", "tuv", "wxyz"};
        std::vector<std::string> result;
        dfs(letters, digits, 0, "", result);
        return result;
    }

    void dfs(const std::array<std::string, 8>& letters, const std::string& digits, int pos,
             const std::string& curr, std::vector<std::string>& result) {
        if (pos == digits.size()) {
            if (curr.size() > 0) {
                result.push_back(curr);
            }
            return;
        }
        for (char c : letters[digits[pos] - '2']) {
            dfs(letters, digits, pos + 1, curr + c, result);
        }
    }
};

int main() {
    return 0;
}
