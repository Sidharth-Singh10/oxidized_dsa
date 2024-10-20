//https://leetcode.com/problems/remove-nth-node-from-end-of-list/
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = &mut head;

        for _ in 0..n {
            fast = fast.map(|n| n.next)?;
        }

        while let Some(f) = fast {
            fast = f.next;
            slow = &mut slow.as_mut()?.next;
        }

        *slow = slow.as_mut().and_then(|n| n.next.take());

        head
    }
}
