public class Solution {
    public bool IsPalindrome(string s) {
        int l = 0;
        int r = s.Length - 1;
        while (l < r) {
            while (l < r && !char.IsAsciiLetterOrDigit(s[l])) {
                l++;
            }
            while (l < r && !char.IsAsciiLetterOrDigit(s[r])) {
                r--;
            }
            if (char.ToLower(s[l]) != char.ToLower(s[r])) {
                return false;
            }
            l++;
            r--;
        }
        return true;
    }
}

