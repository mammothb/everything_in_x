public class Solution
{
    public List<List<string>> SolveNQueens(int n)
    {
        List<List<string>> result = [];
        char[][] curr = new char[n][];
        for (int i = 0; i < n; i++)
        {
            curr[i] = new string('.', n).ToCharArray();
        }
        Dfs(0, n, 0, 0, 0, curr, result);
        return result;
    }

    void Dfs(int col, int n, int rows, int pos, int neg, char[][] curr, List<List<string>> result)
    {
        if (col == n)
        {
            result.Add([.. curr.Select(row => new string(row))]);
        }
        for (int i = 0; i < n; i++)
        {
            if (
                ((rows >> i) & 1) == 1
                || ((pos >> (i + col)) & 1) == 1
                || ((neg >> (n + i - col)) & 1) == 1
            )
            {
                continue;
            }
            curr[i][col] = 'Q';
            Dfs(
                col + 1,
                n,
                rows | (1 << i),
                pos | (1 << (i + col)),
                neg | (1 << (n + i - col)),
                curr,
                result
            );
            curr[i][col] = '.';
        }
    }
}
