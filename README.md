cargo fmt -p algorithm
cargo test trie -p algorithm

git remote add gitee git@gitee.com:hzw-the-programmer/algorithm.git

cargo expand utils::binary_tree

rustup +nightly component add miri
cargo +nightly miri test singly_linked_list::tests::basics
