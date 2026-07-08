public class Solution {
    const int Size = 9;
    const int Bsize = 3;

    public bool IsValidSudoku(char[][] board) {
        int[] rows = new int[9];
        int[] cols = new int[9];
        int[] blks = new int[9];
        for (int i = 0; i < Size; ++i) {
            for (int j = 0; j < Size; ++j) {
                if (board[i][j] == '.') {
                    continue;
                }
                int offset = 1 << (int)(board[i][j] - '0');
                if (((rows[i] & offset) | (cols[j] & offset) | (blks[BlkIdx(i, j)] & offset)) !=
                    0) {
                    return false;
                }
                rows[i] |= offset;
                cols[j] |= offset;
                blks[BlkIdx(i, j)] |= offset;
            }
        }
        return true;
    }

    private int BlkIdx(int i, int j) {
        return (i / Bsize) * Bsize + j / Bsize;
    }
}

