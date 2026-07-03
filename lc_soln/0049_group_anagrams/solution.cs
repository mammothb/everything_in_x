public class Solution {
    public List<List<string>> GroupAnagrams(string[] strs) {
        var variants = new Dictionary<string, List<string>>();
        foreach (string s in strs) {
            var freq = new int[26];
            foreach (char c in s) {
                freq[c - 'a']++;
            }
            string key = string.Join("#", freq);
            if (!variants.ContainsKey(key)) {
                variants[key] = new List<string>();
            }
            variants[key].Add(s);
        }
        return new List<List<string>>(variants.Values);
    }
}
