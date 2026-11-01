public class Solution
{
    public List<List<string>> Partition(string s)
    {
        List<List<string>> result = [];
        List<string> curr = [];
        Dfs(s, 0, curr, result);
        return result;
    }

    void Dfs(string s, int i, List<string> curr, List<List<string>> result)
    {
        if (i >= s.Length)
        {
            result.Add([.. curr]);
            return;
        }
        for (int j = i; j < s.Length; j++)
        {
            if (IsPalindrome(s, i, j))
            {
                curr.Add(s.Substring(i, j - i + 1));
                Dfs(s, j + 1, curr, result);
                curr.RemoveAt(curr.Count - 1);
            }
        }
    }

    bool IsPalindrome(string s, int l, int r)
    {
        while (l < r)
        {
            if (s[l] != s[r])
            {
                return false;
            }
            l++;
            r--;
        }
        return true;
    }
}
