struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }

        // 1. Find the total length of the list
        let mut len = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        // 2. Split the list into two halves
        let mut curr = head.as_mut();
        for _ in 0..(len - 1) / 2 {
            if let Some(node) = curr {
                curr = node.next.as_mut();
            }
        }

        let second_half = curr.and_then(|node| node.next.take());

        // 3. Reverse the second half
        let mut prev = None;
        let mut curr = second_half;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        // 4. Merge the first half and reversed second half
        let mut first = head.as_mut();
        let mut second = prev;

        while let Some(mut second_node) = second {
            if let Some(first_node) = first {
                second = second_node.next.take();
                second_node.next = first_node.next.take();
                first_node.next = Some(second_node);

                // Move first pointer two steps forward
                first = first_node.next.as_mut().and_then(|n| n.next.as_mut());
            } else {
                break;
            }
        }
    }
}

fn main() {}
