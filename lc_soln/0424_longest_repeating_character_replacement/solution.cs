public class Solution
{
    public int CharacterReplacement(string s, int k)
    {
        int result = 0;
        int start = 0;
        int high = 0;
        var counter = new Dictionary<char, int>();
        for (int i = 0; i < s.Length; ++i)
        {
            if (!counter.TryAdd(s[i], 1))
            {
                counter[s[i]]++;
            }
            high = int.Max(high, counter[s[i]]);
            while (i - start + 1 - high > k)
            {
                counter[s[start]]--;
                start++;
            }
            result = int.Max(result, i - start + 1);
        }
        return result;
    }
}
