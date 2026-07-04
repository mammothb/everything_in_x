public class Solution {
    public int[] TopKFrequent(int[] nums, int k) {
        var counter = new Dictionary<int, int>();
        foreach (int num in nums) {
            if (counter.ContainsKey(num)) {
                counter[num]++;
            } else {
                counter[num] = 1;
            }
        }
        var buckets = new Dictionary<int, List<int>>();
        foreach (KeyValuePair<int, int> kvp in counter) {
            if (!buckets.TryGetValue(kvp.Value, out List<int> list)) {
                list = new List<int>();
                buckets[kvp.Value] = list;
            }
            list.Add(kvp.Key);
        }
        var result = new List<int>();
        for (int i = nums.Count(); i > 0; --i) {
            if (buckets.TryGetValue(i, out List<int> bucket)) {
                foreach (int num in bucket) {
                    result.Add(num);
                    if (result.Count() == k) {
                        return result.ToArray();
                    }
                }
            }
        }
        return result.ToArray();
    }
}
