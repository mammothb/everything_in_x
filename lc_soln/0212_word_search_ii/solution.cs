public class TrieNode
{
    public TrieNode[] children = new TrieNode[26];
    public string word = "";
    public int refCount = 0;

    public void Add(string word)
    {
        TrieNode curr = this;
        curr.refCount++;
        foreach (char c in word)
        {
            int idx = c - 'a';
            if (curr.children[idx] is null)
            {
                curr.children[idx] = new TrieNode();
            }
            curr = curr.children[idx];
            curr.refCount++;
        }
        curr.word = word;
    }

    public void Remove(string word)
    {
        TrieNode curr = this;
        curr.refCount--;
        foreach (char c in word)
        {
            int idx = c - 'a';
            TrieNode next = curr.children[idx];
            if (next is null)
            {
                return;
            }
            next.refCount--;
            if (next.refCount == 0)
            {
                curr.children[idx] = null;
                return;
            }
            curr = next;
        }
    }
}

public class Solution
{
    public List<string> FindWords(char[][] board, string[] words)
    {
        var root = new TrieNode();
        foreach (string word in words)
        {
            root.Add(word);
        }

        List<string> result = [];
        int nr = board.Length;
        int nc = board[0].Length;
        for (int i = 0; i < nr; i++)
        {
            for (int j = 0; j < nc; j++)
            {
                if (root.refCount == 0)
                {
                    return result;
                }
                Dfs(board, nr, nc, i, j, root, root, result);
            }
        }
        return result;
    }

    void Dfs(
        char[][] board,
        int nr,
        int nc,
        int row,
        int col,
        TrieNode root,
        TrieNode node,
        List<string> result
    )
    {
        if (!(0 <= row && row < nr && 0 <= col && col < nc))
        {
            return;
        }
        char c = board[row][col];
        if (c == '#')
        {
            return;
        }
        int idx = c - 'a';
        if (node.children[idx] is null)
        {
            return;
        }

        board[row][col] = '#';
        TrieNode child = node.children[idx];
        string word = child.word;
        if (!string.IsNullOrEmpty(word))
        {
            child.word = "";
            result.Add(word);
            root.Remove(word);
        }
        Dfs(board, nr, nc, row - 1, col, root, child, result);
        Dfs(board, nr, nc, row + 1, col, root, child, result);
        Dfs(board, nr, nc, row, col - 1, root, child, result);
        Dfs(board, nr, nc, row, col + 1, root, child, result);
        board[row][col] = c;
    }
}
