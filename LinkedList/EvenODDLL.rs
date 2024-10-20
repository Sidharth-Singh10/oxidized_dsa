//https://leetcode.com/problems/odd-even-linked-list/
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Edge case: list has at most 1 element.
        if head.as_ref()?.next.is_none() {
            return head;
        }

        // Store the heads of the odd and even parts.
        let mut even_head = head.as_mut()?.next.take();
        let mut slow = &mut head;
        let mut fast = &mut even_head;

        // Iterate over all even nodes.
        while fast.is_some() && fast.as_ref()?.next.is_some() {
            // Save the next odd node and move to the next even one.
            let mut next_odd_node = fast.as_mut()?.next.take();
            let next_even_node = next_odd_node.as_mut()?.next.take();

            // Link the current and next even nodes.
            fast.as_mut()?.next = next_even_node;
            fast = &mut fast.as_mut()?.next;

            slow.as_mut()?.next = next_odd_node;
            slow = &mut slow.as_mut()?.next;
        }

        // Link the end of the odd node list to the start of the even node list.
        slow.as_mut()?.next = even_head;
        head
    }
}
