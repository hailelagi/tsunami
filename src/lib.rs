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

// Implement ETS as a library in Rust that can be plugged in to our runtime by writing BIFs that interact with the implementation. 
// Ideally the implementation shouldn't be dependent on runtime specifics like the scheduler, processes, or term representation. 
// If it turns out that such specifics are critical to the implementation though, we can revisit
// Should make use of safe Rust abstractions where possible
// Should provide futures-based APIs for long-running functions (e.g. ets:insert/2 with a list of objects to insert). 
//Functions which are "short enough" don't need to be written as futures, but if implementation is easier that way, it's fine. NOTE: It is essential that these futures do not attempt to hold locks across yield points, if Rust will even allow that in its type system.
// ETS tables should be reference counted, and likely the values they contain as well, so that access by multiple processes is safe, and garbage collection of the tables and their contents can happen naturally.
// While correctness is the most important property, attempts should be made to keep the implementation as performance-sensitive as possible, since ETS is so heavily relied upon.

// Ideally, we want to use lock-free data structures when the table is public, and so concurrent readers/writers are present, 
//and standard non-thread safe data structures when the table is private. BEAM went down the path of using read/write locks
//when the first SMP-enabled emulator was implemented, but quite a lot has evolved in the world of lock-free data structures 
//since then. Since they went down that path, they ended up having to also implement lock groups, which ensure that contention 
//on the locks is reduced by breaking up scheduler threads across a group of locks, where each thread only acquires the locks 
//necessary to read/write the area of the table it is working in. This is more or less where ETS remains today.

// There are a variety of lock-free data structure crates available for Rust, but we'll need to vet them on a few requirements,
// and potentially fork and modify them to suit our needs:

// How well maintained/used are they
// Do they provide the semantics we require
// Do they support custom allocation or will we need to patch that in
// Do they support dynamic resizing (this is primarily an issue for hash tables)
// Do they provide the necessary primitives we need to implement the Erlang ets module APIs

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
