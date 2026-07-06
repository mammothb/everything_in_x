class Solution {
public:
    string encode(vector<string>& strs) {
        std::stringstream ss;
        for (int i = 0; i < strs.size(); ++i) {
            ss << std::to_string(strs[i].size()) << ",";
        }
        ss << '#';
        for (const std::string& s : strs) {
            ss << s;
        }
        return ss.str();
    }

    vector<string> decode(string s) {
        if (s.empty()) {
            return {};
        }
        std::vector<int> lengths;
        int i = 0;
        while (s[i] != '#') {
            int j = i;
            while (s[j] != ',') {
                ++j;
            }
            lengths.push_back(std::stoi(s.substr(i, j - i)));
            i = j + 1;
        }
        i++;
        std::vector<std::string> result;
        for (int length : lengths) {
            result.push_back(s.substr(i, length));
            i += length;
        }
        return result;
    }
};

int main() {
    return 0;
}
