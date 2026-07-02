public class Solution {
    public bool IsAnagram(string s, string t) {
        if (s.Length != t.Length) {
            return false;
        }
        var counterS = new Dictionary<char, int>();
        var counterT = new Dictionary<char, int>();
        foreach (char c in s) {
            counterS[c] = counterS.GetValueOrDefault(c) + 1;
        }
        foreach (char c in t) {
            counterT[c] = counterT.GetValueOrDefault(c) + 1;
        }
        foreach (KeyValuePair<char, int> kvp in counterS) {
            if (counterT.GetValueOrDefault(kvp.Key) != kvp.Value) {
                return false;
            }
        }
        foreach (KeyValuePair<char, int> kvp in counterT) {
            if (counterS.GetValueOrDefault(kvp.Key) != kvp.Value) {
                return false;
            }
        }
        return true;
    }
}

