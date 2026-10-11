public class TrieNode
{
    public TrieNode[] children = new TrieNode[26];
    public bool endOfWord = false;
}

public class PrefixTree
{
    readonly TrieNode root = new();

    public void Insert(string word)
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
        TrieNode curr = root;
        foreach (char c in word)
        {
            int idx = c - 'a';
            if (curr.children[idx] is null)
            {
                return false;
            }
            curr = curr.children[idx];
        }
        return curr.endOfWord;
    }

    public bool StartsWith(string prefix)
    {
        TrieNode curr = root;
        foreach (char c in prefix)
        {
            int idx = c - 'a';
            if (curr.children[idx] is null)
            {
                return false;
            }
            curr = curr.children[idx];
        }
        return true;
    }
}
