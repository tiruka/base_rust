# base_rust

This branch is for learning Rust Atomcis and Locks.

I take notes for what I learned and each dirctory have some codes for sure.

## Chapter1

- Interior Mutibility:
  It does not mean mutability and immutability. It is better to use terms "shared" and "exclusive". Shared Reference "&T" indicates we can share T with others by copy. Exclusive Reference "&mut T".

- p16 i32, bool, str などは send, sync を auto trait するので、これらの要素だけでできた struct も自動で sync, send を備える。もしこれを防ぎたい場合は、std::marker::PhantomData<T>を利用して、Cell などを入れる。サイズを使わずに防ぐことができる。Cell は Sync ではないから。

https://github.com/m-ou-se/rust-atomics-and-locks/tree/main

## Chapter4

Sync というのは、他のスレッドでデータを同期させたい場合に必要な trait. Send というのは、他のスレッドへデータを共有したい場合に必要な trait.
両者の違いは、一つのスレッドしか排他的にアクセスしないなら、Send のみで良い。しかし、同時に複数スレッドが参照するなら、Sync が必要。
