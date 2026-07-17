public class Solution
{
    public string MinWindow(string s, string t)
    {
        var counter = new Dictionary<char, int>();
        foreach (char c in t)
        {
            counter[c] = counter.GetValueOrDefault(c, 0) + 1;
        }
        int need = counter.Count;
        int start = 0;
        int best = int.MaxValue;
        int best_start = 0;
        for (int i = 0; i < s.Length; ++i)
        {
            if (counter.ContainsKey(s[i]))
            {
                counter[s[i]]--;
                if (counter[s[i]] == 0)
                {
                    need--;
                }
            }
            while (start <= i && (!counter.TryGetValue(s[start], out int v) || v < 0))
            {
                if (counter.ContainsKey(s[start]))
                {
                    counter[s[start]]++;
                }
                start++;
            }
            if (need == 0 && i - start + 1 < best)
            {
                best = i - start + 1;
                best_start = start;
            }
        }
        return best == int.MaxValue ? "" : s.Substring(best_start, best);
    }
}
