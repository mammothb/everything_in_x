public class Solution
{
    static readonly int[] _dirs = { 0, 1, 0, -1, 0 };

    public bool Exist(char[][] board, string word)
    {
        int nr = board.Length;
        int nc = board[0].Length;
        for (int i = 0; i < nr; i++)
        {
            for (int j = 0; j < nc; j++)
            {
                if (Dfs(board, i, j, 0, word))
                {
                    return true;
                }
            }
        }
        return false;
    }

    bool Dfs(char[][] board, int i, int j, int pos, string word)
    {
        if (pos == word.Length)
        {
            return true;
        }
        int nr = board.Length;
        int nc = board[0].Length;
        if (!(0 <= i && i < nr && 0 <= j && j < nc))
        {
            return false;
        }
        char c = board[i][j];
        if (c == '.' || c != word[pos])
        {
            return false;
        }
        board[i][j] = '.';
        for (int k = 0; k < 4; k++)
        {
            int next_i = i + _dirs[k];
            int next_j = j + _dirs[k + 1];
            if (Dfs(board, next_i, next_j, pos + 1, word))
            {
                return true;
            }
        }
        board[i][j] = c;

        return false;
    }
}
