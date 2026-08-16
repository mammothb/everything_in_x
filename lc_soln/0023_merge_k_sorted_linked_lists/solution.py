import heapq

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        h = [(node.val, i) for i, node in enumerate(lists) if node is not None]
        heapq.heapify(h)
        dummy = ListNode()
        curr = dummy
        while h:
            _, i = heapq.heappop(h)
            curr.next = lists[i]
            curr = curr.next
            lists[i] = lists[i].next
            if lists[i] is not None:
                heapq.heappush(h, (lists[i].val, i))

        return dummy.next


def main(): ...


if __name__ == "__main__":
    main()
