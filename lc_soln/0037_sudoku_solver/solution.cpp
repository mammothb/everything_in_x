#include <string>
#include <utility>
#include <vector>

using namespace std;

class Solution {
    static constexpr int kDim = 9;
    static constexpr int kSDim = 3;
    static constexpr string kNums = "123456789";

    vector<int> rows;
    vector<int> cols;
    vector<int> boxs;
    vector<pair<int, int>> blanks;

public:
    void solveSudoku(vector<vector<char>>& board) {
        rows.assign(kDim, 0);
        cols.assign(kDim, 0);
        boxs.assign(kDim, 0);
        for (int i = 0; i < kDim; ++i) {
            for (int j = 0; j < kDim; ++j) {
                char c = board[i][j];
                if (c == '.') {
                    blanks.push_back({i, j});
                } else {
                    int offset = c - '1';
                    rows[i] |= 1 << offset;
                    cols[j] |= 1 << offset;
                    boxs[boxIdx(i, j)] |= 1 << offset;
                }
            }
        }
        solve(board, 0);
    }

    inline int boxIdx(const int i, const int j) const {
        return (i / kSDim) * kSDim + j / kSDim;
    }

    inline bool isValid(const int i, const int j, const int k,
                        const int offset) const {
        return !(((rows[i] >> offset) & 1) | ((cols[j] >> offset) & 1) |
                  ((boxs[k] >> offset) & 1));
    }

    bool solve(vector<vector<char>>& board, const int idx) {
        if (idx == blanks.size()) {
            return true;
        }
        auto [i, j] = blanks[idx];
        int k = boxIdx(i, j);
        for (const char& num : kNums) {
            int offset = num - '1';
            if (isValid(i, j, k, offset)) {
                board[i][j] = num;
                rows[i] |= 1 << offset;
                cols[j] |= 1 << offset;
                boxs[k] |= 1 << offset;
                if (solve(board, idx + 1)) {
                    return true;
                }
                board[i][j] = '.';
                rows[i] &= ~(1 << offset);
                cols[j] &= ~(1 << offset);
                boxs[k] &= ~(1 << offset);
            }
        }
        return false;
    }
};

