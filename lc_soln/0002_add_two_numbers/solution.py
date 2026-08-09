# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        dummy = ListNode()
        curr = dummy
        total = 0
        while l1 is not None and l2 is not None:
            total += l1.val + l2.val
            l1 = l1.next
            l2 = l2.next
            node = ListNode(val=total % 10)
            total //= 10
            curr.next = node
            curr = curr.next
        while l1 is not None:
            total += l1.val
            l1 = l1.next
            node = ListNode(val=total % 10)
            total //= 10
            curr.next = node
            curr = curr.next
        while l2 is not None:
            total += l2.val
            l2 = l2.next
            node = ListNode(val=total % 10)
            total //= 10
            curr.next = node
            curr = curr.next
        if total > 0:
            curr.next = ListNode(val=total)
        return dummy.next


def main(): ...


if __name__ == "__main__":
    main()
