public class Solution
{
    public int LengthOfLongestSubstring(string s)
    {
        int result = 0;
        int start = 0;
        var counter = new Dictionary<char, int>();
        for (int i = 0; i < s.Length; ++i)
        {
            while (counter.TryGetValue(s[i], out int val) && val > 0)
            {
                counter[s[start]]--;
                start++;
            }
            counter[s[i]] = counter.GetValueOrDefault(s[i]) + 1;
            result = int.Max(result, i - start + 1);
        }
        return result;
    }
}
