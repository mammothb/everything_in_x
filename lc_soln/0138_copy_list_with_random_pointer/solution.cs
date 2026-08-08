/*
// Definition for a Node.
public class Node {
    public int val;
    public Node next;
    public Node random;

    public Node(int _val) {
        val = _val;
        next = null;
        random = null;
    }
}
*/

public class Solution
{
    public Node copyRandomList(Node head)
    {
        if (head == null)
        {
            return null;
        }
        var newHead = new Node(head.val);
        var oldToNew = new Dictionary<Node, Node>();
        oldToNew.Add(head, newHead);

        Node curr = head;
        Node newCurr = newHead;
        while (curr != null)
        {
            if (curr.next != null)
            {
                Node newNext;
                if (oldToNew.TryGetValue(curr.next, out Node next))
                {
                    newNext = next;
                }
                else
                {
                    newNext = new Node(curr.next.val);
                    oldToNew.Add(curr.next, newNext);
                }
                newCurr.next = newNext;
            }
            if (curr.random != null)
            {
                Node newRandom;
                if (oldToNew.TryGetValue(curr.random, out Node random))
                {
                    newRandom = random;
                }
                else
                {
                    newRandom = new Node(curr.random.val);
                    oldToNew.Add(curr.random, newRandom);
                }
                newCurr.random = newRandom;
            }
            curr = curr.next;
            newCurr = newCurr.next;
        }
        return newHead;
    }
}
