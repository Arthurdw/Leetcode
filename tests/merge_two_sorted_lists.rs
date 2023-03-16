// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    fn merge(list1: Box<ListNode>, list2: Box<ListNode>) -> Box<ListNode> {
        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        let mut list1 = Some(list1);
        let mut list2 = Some(list2);

        loop {
            let list_val1 = list1.as_ref().unwrap();
            let list_val2 = list2.as_ref().unwrap();

            if list_val1.val < list_val2.val {
                current.next = Some(Box::new(ListNode::new(list_val1.val)));
                current = current.next.as_mut().unwrap();

                if let Some(next) = list_val1.next.as_ref() {
                    list1 = Some(next.clone());
                } else {
                    current.next = list2;
                    break;
                }
            } else {
                current.next = Some(Box::new(ListNode::new(list_val2.val)));
                current = current.next.as_mut().unwrap();

                if let Some(next) = list_val2.next.as_ref() {
                    list2 = Some(next.clone());
                } else {
                    current.next = list1;
                    break;
                }
            }
        }

        head.next.unwrap()
    }

    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(li1), Some(li2)) => Some(Solution::merge(li1, li2)),
            (Some(li1), _) => Some(li1),
            (_, Some(li2)) => Some(li2),
            _ => None
        }
    }
}

#[test]
fn test() {
    let mut list1 = Box::new(ListNode::new(1));
    list1.next = Some(Box::new(ListNode::new(2)));
    list1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut list2 = Box::new(ListNode::new(1));
    list2.next = Some(Box::new(ListNode::new(3)));
    list2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut out = Box::new(ListNode::new(1));
    out.next = Some(Box::new(ListNode::new(1)));
    out.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    out.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    out.next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    out.next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    assert_eq!(Solution::merge_two_lists(Some(list1), Some(list2)), Some(out));
}