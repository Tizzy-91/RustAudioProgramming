
// refresher
// pointers are references to a memory location
// what they store is an address to a location in memory
// & is the reference operator, allows us to borrow a value
// Strings and Vectors are Smart pointers
    // they own the data
    // they have functions to manipulate that data
    // can track ownership of data
// stack:  LIFO and must have defined size.
// heap:  can grow and shrink as needed.  slower than stack

fn main() {
    // BOX smart pointer stores on the heap rather than the stack
    // BOX is normally for storing large amounts of data on 
    // the heap, and then returning a pointer to that data on the stack

    let b_int1 = Box::new(5);
    println!("b_int1 = {}", b_int1);
    let b_int2 = b_int1;
    // b_int1 is no longer valid, it was moved to b_int2
    // println!("b_int1 = {}", b_int1);  // error
    println!("b_int2 = {}", b_int2);


    // errors because infinite possible size
    // errors because rust doesn't like null vals
    // struct TreeNode<T> {
    //     pub key: T,
    //     pub left: TreeNode<T>,
    //     pub right: TreeNode<T>,
    // }

    struct TreeNode<T> {
        pub value: T,
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
    }

    impl<T> TreeNode<T> {
        pub fn new(value: T) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self // returning our self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self // returning our self
        }
    }

    let root: TreeNode<i32> = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    println!("root = {}", root.value);
}
