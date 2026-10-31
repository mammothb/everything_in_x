class Solution {
    std::array<int, 5> dirs = {0, 1, 0, -1, 0};
public:
    bool exist(vector<vector<char>>& board, string word) {
        int nr = board.size();
        int nc = board[0].size();
        for (int i = 0; i < nr; i++) {
            for (int j = 0; j < nc; j++) {
                if (dfs(board, i, j, 0, word)) {
                    return true;
                }
            }
        }
        return false;
    }

    bool dfs(std::vector<std::vector<char>>& board, int i, int j, int pos, const std::string& word) {
        if (pos == word.size()) {
            return true;
        }
        int nr = board.size();
        int nc = board[0].size();
        if (!(0 <= i && i < nr && 0 <= j && j < nc)) {
            return false;
        }
        char c = board[i][j];
        if (c == '.' || c != word[pos]) {
            return false;
        }
        board[i][j] = '.';
        for (int k = 0; k < 4; k++) {
            int next_i = i + dirs[k];
            int next_j = j + dirs[k + 1];
            if (dfs(board, next_i, next_j, pos + 1, word)) {
                return true;
            }
        }
        board[i][j] = c;
        return false;
    }
};

int main() {
    return 0;
}
