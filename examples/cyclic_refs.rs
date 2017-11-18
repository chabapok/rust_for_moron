/*

 How can I implement a graph or other data structure that contains cycles?

There are at least four options (discussed at length in Too Many Linked Lists):

    You can implement it using Rc and Weak to allow shared ownership of nodes, although this approach pays the cost of memory management.
    You can implement it using unsafe code using raw pointers. This will be more efficient, but bypasses Rust’s safety guarantees.
    Using vectors and indices into those vectors. There are several available examples and explanations of this approach.
    Using borrowed references with UnsafeCell. There are explanations and code available for this approach.

    https://github.com/chabapok/r4cppp/tree/master/graphs
    (там с примерами)


    пример кода
    https://play.rust-lang.org/?gist=c1b0d133e56e5c1e4ea4ed2ecd7dd28a&version=stable

*/