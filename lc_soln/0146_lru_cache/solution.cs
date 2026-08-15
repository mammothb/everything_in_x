public class LRUCacheNode(int key = -1, int val = -1)
{
    public int Key { get; set; } = key;
    public int Val { get; set; } = val;
    public LRUCacheNode Prev { get; set; } = null;
    public LRUCacheNode Next { get; set; } = null;
}

public class LRUCache
{
    private int _size = 0;
    private readonly Dictionary<int, LRUCacheNode> _keyToNode = [];
    private readonly int _capacity;
    private readonly LRUCacheNode _head;
    private readonly LRUCacheNode _tail;

    public LRUCache(int capacity)
    {
        _capacity = capacity;
        _head = new LRUCacheNode();
        _tail = new LRUCacheNode();
        _head.Next = _tail;
        _tail.Prev = _head;
    }

    static LRUCacheNode Remove(LRUCacheNode node)
    {
        node.Prev.Next = node.Next;
        node.Next.Prev = node.Prev;
        node.Prev = null;
        node.Next = null;
        return node;
    }

    static LRUCacheNode InsertAfter(LRUCacheNode node, LRUCacheNode target)
    {
        node.Prev = target;
        node.Next = target.Next;
        target.Next.Prev = node;
        target.Next = node;
        return node;
    }

    public int Get(int key)
    {
        if (_keyToNode.TryGetValue(key, out LRUCacheNode node))
        {
            node = Remove(node);
            node = InsertAfter(node, _head);
            return node.Val;
        }
        return -1;
    }

    public void Put(int key, int value)
    {
        if (_keyToNode.TryGetValue(key, out LRUCacheNode node))
        {
            node.Val = value;
            node = Remove(node);
            node = InsertAfter(node, _head);
        }
        else
        {
            if (_size == _capacity)
            {
                LRUCacheNode removed = Remove(_tail.Prev);
                _keyToNode.Remove(removed.Key);
                _size--;
            }
            node = new LRUCacheNode(key, value);
            node = InsertAfter(node, _head);
            _keyToNode[key] = node;
            _size++;
        }
    }
}
