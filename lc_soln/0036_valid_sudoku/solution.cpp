const int SIZE = 9;
const int BSIZE = 3;

int blk_idx(int i, int j) { return (i / BSIZE) * BSIZE + j / BSIZE; }

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        std::vector<int> rows(SIZE, 0);
        std::vector<int> cols(SIZE, 0);
        std::vector<int> blks(SIZE, 0);
        for (int i = 0; i < SIZE; ++i) {
            for (int j = 0; j < SIZE; ++j) {
                if (board[i][j] == '.') {
                    continue;
                }
                int offset = 1 << (board[i][j] - '0');
                if ((rows[i] & offset) != 0 || (cols[j] & offset) != 0 ||
                    (blks[blk_idx(i, j)] & offset) != 0) {
                    return false;
                }
                rows[i] |= offset;
                cols[j] |= offset;
                blks[blk_idx(i, j)] |= offset;
            }
        }
        return true;
    }
};

int main() {
    return 0;
}
