use crate::util::linked_list::ListNode;

fn fast_and_slow_pointer(mut head: Option<Box<ListNode>>) -> i32 {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    let mut ans = 0;

    while let Some(f) = fast {
        if f.next.is_none() {
            break;
        }
        slow = slow.unwrap().next.as_ref();
        fast = f.next.as_ref().unwrap().next.as_ref();
        // do logic
    }

    ans
}

fn reverse_linked_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}
