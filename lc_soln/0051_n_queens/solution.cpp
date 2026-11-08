class Solution {
   public:
    vector<vector<string>> solveNQueens(int n) {
        std::vector<std::vector<std::string>> result;
        std::vector<std::vector<bool>> curr(n, std::vector<bool>(n, false));
        dfs(0, n, curr, result);
        return result;
    }

    void dfs(int col, int n, std::vector<std::vector<bool>>& curr,
             std::vector<std::vector<std::string>>& result) {
        if (col == n) {
            std::vector<std::string> tmp(n);
            for (int i = 0; i < n; i++) {
                std::stringstream ss;
                for (int j = 0; j < n; j++) {
                    if (curr[i][j]) {
                        ss << 'Q';
                    } else {
                        ss << '.';
                    }
                }
                tmp[i] = ss.str();
            }
            result.push_back(tmp);
            return;
        }
        for (int i = 0; i < n; i++) {
            if (!isValid(col, n, i, curr)) {
                continue;
            }
            curr[i][col] = true;
            dfs(col + 1, n, curr, result);
            curr[i][col] = false;
        }
    }

    bool isValid(int col, int n, int row, const std::vector<std::vector<bool>>& curr) {
        for (int j = 0; j < col; j++) {
            if (curr[row][j]) {
                return false;
            }
        }
        int i = row - 1;
        int j = col - 1;
        while (i >= 0 && j >= 0) {
            if (curr[i][j]) {
                return false;
            }
            i--;
            j--;
        }
        i = row + 1;
        j = col - 1;
        while (i < n && j >= 0) {
            if (curr[i][j]) {
                return false;
            }
            i++;
            j--;
        }
        return true;
    }
};

int main() {
    return 0;
}
