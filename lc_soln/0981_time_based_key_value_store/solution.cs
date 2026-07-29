public class TimeMap
{
    private Dictionary<string, List<(int timestamp, string value)>> data = new();

    public TimeMap() { }

    public void Set(string key, string value, int timestamp)
    {
        if (data.TryGetValue(key, out List<(int timestamp, string value)> list))
        {
            list.Add((timestamp, value));
        }
        else
        {
            data.Add(key, [(timestamp, value)]);
        }
    }

    public string Get(string key, int timestamp)
    {
        if (data.TryGetValue(key, out List<(int timestamp, string value)> list))
        {
            int l = 0;
            int r = list.Count - 1;
            int idx = -1;
            while (l <= r)
            {
                int mid = l + (r - l) / 2;
                if (list[mid].timestamp <= timestamp)
                {
                    idx = mid;
                    l = mid + 1;
                }
                else
                {
                    r = mid - 1;
                }
            }
            return idx == -1 ? "" : list[idx].value;
        }
        return "";
    }
}
