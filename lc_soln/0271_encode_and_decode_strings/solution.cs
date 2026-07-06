public class Solution {

    public string Encode(IList<string> strs) {
        var sb = new StringBuilder();
        for (int i = 0; i < strs.Count; ++i) {
            if (i != 0) {
                sb.Append(',');
            }
            sb.Append(strs[i].Length);
        }
        sb.Append('#');
        foreach (string s in strs) {
            sb.Append(s);
        }
        return sb.ToString();
    }

    public List<string> Decode(string s) {
        string[] parts = s.Split('#', 2);
        int[] lengths = Array.ConvertAll(parts[0].Split(','), int.Parse);
        List<string> result = [];
        int start = 0;
        foreach (int length in lengths) {
            result.Add(parts[1].Substring(start, length));
            start += length;
        }
        return result;
   }
}
