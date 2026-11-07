public class Solution
{
    public List<string> LetterCombinations(string digits)
    {
        List<string> result = [];
        string[] letters = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        Dfs(letters, digits, 0, "", result);
        return result;
    }

    void Dfs(string[] letters, string digits, int pos, string curr, List<string> result)
    {
        if (pos == digits.Length)
        {
            if (curr.Length > 0)
            {
                result.Add(curr);
            }
            return;
        }
        foreach (char c in letters[digits[pos] - '2'])
        {
            Dfs(letters, digits, pos + 1, curr + c, result);
        }
    }
}
