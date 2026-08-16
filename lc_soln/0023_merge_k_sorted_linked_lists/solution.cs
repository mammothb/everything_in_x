/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

public class Solution
{
    public ListNode MergeKLists(ListNode[] lists)
    {
        if (lists.Length == 0)
        {
            return null;
        }
        var q = new LinkedList<ListNode>(lists);
        while (q.Count > 1)
        {
            ListNode l1 = q.First.Value;
            q.RemoveFirst();
            ListNode l2 = q.First.Value;
            q.RemoveFirst();
            ListNode l3 = Merge(l1, l2);
            q.AddLast(l3);
        }
        return q.First.Value;
    }

    ListNode Merge(ListNode l1, ListNode l2)
    {
        var dummy = new ListNode();
        ListNode curr = dummy;
        while (l1 is not null && l2 is not null)
        {
            if (l1.val < l2.val)
            {
                curr.next = l1;
                l1 = l1.next;
            }
            else
            {
                curr.next = l2;
                l2 = l2.next;
            }
            curr = curr.next;
        }
        if (l1 is not null)
        {
            curr.next = l1;
        }
        if (l2 is not null)
        {
            curr.next = l2;
        }
        return dummy.next;
    }
}
