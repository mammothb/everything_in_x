public class KthLargest
{
    readonly PriorityQueue<int, int> _q;
    readonly int _k;

    public KthLargest(int k, int[] nums)
    {
        _k = k;
        _q = new PriorityQueue<int, int>(_k + 1);
        foreach (int num in nums)
        {
            if (_q.Count == _k)
            {
                _q.EnqueueDequeue(num, num);
            }
            else
            {
                _q.Enqueue(num, num);
            }
        }
    }

    public int Add(int val)
    {
        if (_q.Count == _k)
        {
            _q.EnqueueDequeue(val, val);
        }
        else
        {
            _q.Enqueue(val, val);
        }
        return _q.Peek();
    }
}
