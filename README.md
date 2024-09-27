# base_rust

This branch is for learning Rust Atomcis and Locks.

I take notes for what I learned and each dirctory have some codes for sure.

## Chapter1

- Interior Mutibility:
  It does not mean mutability and immutability. It is better to use terms "shared" and "exclusive". Shared Reference "&T" indicates we can share T with others by copy. Exclusive Reference "&mut T".

- p16 i32, bool, str などは send, sync を auto trait するので、これらの要素だけでできた struct も自動で sync, send を備える。もしこれを防ぎたい場合は、std::marker::PhantomData<T>を利用して、Cell などを入れる。サイズを使わずに防ぐことができる。Cell は Sync ではないから。

https://github.com/m-ou-se/rust-atomics-and-locks/tree/main
