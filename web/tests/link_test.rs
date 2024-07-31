#[derive(Debug, Clone)]
struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, value: i32) {
        // if let None = self.head {
        //     self.head = Some(Box::new(ListNode { value, next: None }));
        //     return;
        // }
        // 与 if let Some(mut node) = self.head.as_mut() 是等效的
        // if let Some(mut node) = self.head.as_mut() {
        //     loop {
        //         if node.next.is_some() {
        //             node = node.next.as_mut().unwrap();
        //         } else {
        //             node.next = Some(Box::new(ListNode { value, next: None }));
        //             break;
        //         }
        //     }
        // }

        match self.head.as_mut() {
            None => self.head = Some(Box::new(ListNode { value, next: None })),
            Some(mut node) => {
                // 判断是否为Some类型
                while node.next.is_some() {
                    node = node.next.as_mut().unwrap();
                }

                node.next = Some(Box::new(ListNode { value, next: None }));
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let value = node.value;
            self.head = node.next;
            value
        })
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        let mut current = &self.head;
        for _ in 0..index {
            current = match current {
                Some(node) => &node.next,
                None => return None,
            };
        }

        match current {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }
        length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.value);
            current = &node.next;
        }
    }
}

#[test]
fn test1() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("链表长度: {}", list.len());
    println!("第二个元素: {:?}", list.get(1));

    list.print();

    while let Some(_) = list.pop() {}

    println!("链表是否为空: {}", list.is_empty());
    println!("-----------");
    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("link: {:?}", list);
    println!("链表长度: {}", list.len());
    println!("第二个元素: {:?}", list.get(1));
}
