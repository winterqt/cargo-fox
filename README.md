# cargo-fox

Add foxes to your project.

## installation

```
cargo install cargo-fox
```

## usage

```
cargo fox
```

## what does it actually do?

It takes

```rust
fn main() {
    println!("Hello world!");
}
```

and turns it into

```rust
// a fox.
fn main() {
    println!("Hello world!");
}
```

(Fox locations are chosen at random.)
