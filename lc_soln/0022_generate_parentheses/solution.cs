public class Solution
{
    public List<string> GenerateParenthesis(int n)
    {
        List<string> result = [];
        string curr = "";
        Dfs(0, n, curr, result);
        return result;
    }

    void Dfs(int left, int right, string curr, List<string> result)
    {
        if (left == 0 && right == 0)
        {
            result.Add(curr);
            return;
        }
        if (left > 0)
        {
            Dfs(left - 1, right, curr + ")", result);
        }
        if (right > 0)
        {
            Dfs(left + 1, right - 1, curr + "(", result);
        }
    }
}
