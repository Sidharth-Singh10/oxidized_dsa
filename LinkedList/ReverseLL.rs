    
//https://leetcode.com/problems/reverse-linked-list/

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // If the list is empty, return None
        if head.is_none() {
            return head;
        }

        let mut slow = head;
        let mut fast = slow.as_mut().unwrap().next.take(); // Fast is initialized to the next node
        slow.as_mut().unwrap().next = None; // The head's next should point to None to reverse it

        // Loop until fast is None
        while let Some(mut fast_node) = fast {
            let temp = fast_node.next.take(); // Save fast's next node
            fast_node.next = slow;            // Reverse fast's pointer to point to slow
            slow = Some(fast_node);           // Move slow forward
            fast = temp;                      // Move fast forward
        }

        slow // Return the reversed list
    }
}
