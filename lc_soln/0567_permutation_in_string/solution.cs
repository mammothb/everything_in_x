public class Solution
{
    public bool CheckInclusion(string s1, string s2)
    {
        var counter = new Dictionary<char, int>();
        foreach (char c in s1)
        {
            counter[c] = counter.GetValueOrDefault(c, 0) + 1;
        }
        var curr = new Dictionary<char, int>();
        int start = 0;
        for (int i = 0; i < s2.Length; ++i)
        {
            if (!counter.ContainsKey(s2[i]))
            {
                curr.Clear();
                start = i + 1;
                continue;
            }
            curr[s2[i]] = curr.GetValueOrDefault(s2[i], 0) + 1;
            while (curr[s2[i]] > counter[s2[i]])
            {
                curr[s2[start]]--;
                start++;
            }
            if (i - start + 1 == s1.Length)
            {
                return true;
            }
        }
        return false;
    }
}
