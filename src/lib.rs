/*
- https://www.erlang.org/doc/man/ets.html
- futures based api
- reference counted tables and data
- no locks across yield points
- 

Table types:
1. set - AVL
2. ordered_set - CA Tree with write concurrency
3. bag
4. duplicate_bag
*/

// basic:
// insertion
// deletion
// search

// todo
// :ets public api 
// 

// todo: internal node vs external node?
struct WAVLNode<T: Ord> {
    _value: T,
    _height: usize,
    _rank: usize,
    _left: Option<Box<WAVLNode<T>>>,
    _right: Option<Box<WAVLNode<T>>>,
}

pub struct WAVLTree<T: Ord> {
    _root: Option<Box<WAVLNode<T>>>,
    _length: usize,
}

/*
impl WAVLNode {
    fn rank_difference(&self, WAVLNode) -> usize {
        // todo
    }
}
*/
