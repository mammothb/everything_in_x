public class TrieNode
{
    public TrieNode[] children = new TrieNode[26];
    public bool endOfWord = false;
}

public class WordDictionary
{
    readonly TrieNode root = new();

    public void AddWord(string word)
    {
        TrieNode curr = root;
        foreach (char c in word)
        {
            int idx = c - 'a';
            if (curr.children[idx] is null)
            {
                curr.children[idx] = new TrieNode();
            }
            curr = curr.children[idx];
        }
        curr.endOfWord = true;
    }

    public bool Search(string word)
    {
        return Dfs(root, word, 0);
    }

    bool Dfs(TrieNode node, string word, int pos)
    {
        if (pos == word.Length)
        {
            return node.endOfWord;
        }
        if (word[pos] == '.')
        {
            foreach (TrieNode child in node.children)
            {
                if (child is not null && Dfs(child, word, pos + 1))
                {
                    return true;
                }
            }
            return false;
        }
        int idx = word[pos] - 'a';
        if (node.children[idx] is not null)
        {
            return Dfs(node.children[idx], word, pos + 1);
        }
        return false;
    }
}
