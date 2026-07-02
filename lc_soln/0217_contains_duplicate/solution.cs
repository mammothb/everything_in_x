public class Solution {
    public bool hasDuplicate(int[] nums) {
        var seen = new HashSet<int>();
        foreach (int num in nums) {
            if (seen.Contains(num)) {
                return true;
            }
            seen.Add(num);
        }
        return false;
    }
}

class Program {
    static void Main() {
        var solution = new Solution();
        Console.WriteLine(solution.hasDuplicate([1, 2, 3, 3]));
    }
}
