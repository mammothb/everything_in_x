class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        std::unordered_map<std::string, std::vector<std::string>> variants;
        for (const std::string& s : strs) {
            std::vector<int> freq(26, 0);
            for (char c : s) {
                freq[c - 'a']++;
            }
            std::string key = "";
            for (int f : freq) {
                key += '#' + std::to_string(f);
            }
            variants[key].push_back(s);
        }
        std::vector<std::vector<std::string>> result;
        result.reserve(variants.size());
        for (const auto& [_, val] : variants) {
            result.push_back(std::move(val));
        }
        return result;
    }
};

int main() {
    return 0;
}
