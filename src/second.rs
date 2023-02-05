type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // fn pop(&mut self) -> Option<i32> {
    //     match self.head.take() {
    //         None => None,
    //         Some(node) => {
    //             self.head = node.next;
    //             Some(node.elem)
    //         }
    //     }
    // }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut box_node) = cur_link {
            cur_link = box_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        // assert_eq!(list.pop(), Some(5));
        // assert_eq!(list.pop(), Some(4));

        // // Check exhaustion
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), None);
    }
}
