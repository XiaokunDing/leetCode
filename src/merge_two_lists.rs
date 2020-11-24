// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut retHead: Option<Box<ListNode>> = Some(Box::new(ListNode::new(-1))); // 创建一个头节点
        let mut cur= &mut retHead;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut next = true;
        while next == true {
            match (l1.take(),l2.take()) {  // 取出来比较
                (Some(_l1),None) => {
                    // 只有l1了，后面不再需要遍历
                    if let Some(ref mut _cur) = cur { // ref 禁止移动，mut 确保可以修改
                        _cur.next = Some(_l1);
                    }
                    next = false;
                },
                (None,Some(_l2)) => {
                    // 只有l2了，后面不再需要遍历
                    if let Some(ref mut _cur) = cur {  // ref 禁止移动，mut 确保可以修改
                        _cur.next = Some(_l2);
                    }
                    next = false;
                },
                (Some(mut _l1),Some(mut _l2)) => {  // mut 确保可以修改
                    // 需要比大小了，先比个大小
                    if &_l1.val < &_l2.val {
                        let _next = _l1.next.take();
                        if let Some(ref mut _cur) = cur {  // ref 禁止移动，mut 确保可以修改
                            _cur.next = Some(_l1);
                            cur = &mut _cur.next; // 游标移动
                        }
                        l1 = _next;
                        l2 = Some(_l2);  // 大的放回
                    }else{
                        let _next = _l2.next.take();
                        if let Some(ref mut _cur) = cur {
                            _cur.next = Some(_l2);
                            cur = &mut _cur.next; // 游标移动
                        }
                        l2 = _next;
                        l1 = Some(_l1);  // 大的放回
                    }
                },
                (None,None) => {
                    next = false;
                },
            }
        }

        return retHead.unwrap().next;
    }

}