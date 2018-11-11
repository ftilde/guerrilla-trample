guerilla-trample
================
Demonstration on how guerilla (version 0.1.2) can trample code following very small functions.

In short, we define a very short (3 byte) function, which will be replaced by another function which is far enough away that a relative jump (5 bytes) is required. The 5-3=2 "overhanging" bytes trample code following the replaced function.

More detail:
------------
It works by defining 4 functions which will (hopefully) laid out sequentially:

```rust
pub fn fun1() -> i64 {
    0
}
```
This compiles to `xor rax, rax; retq` => 3 bytes

```rust
pub fn fun2() -> i64 {
    27
}
```
The first two bytes will be trampled and cause a SIGSEGV or SIGILL when executed.

```rust
pub fn padding(mut x: i64) -> i64 {
    x += 1;
    x += 1;
    x += 1;
    // [...] Repeated 200 or so times
    x += 1;
    x
}
```
This causes the following function `replacement` and `fun1` to be separated enough to forbid short jumps and thus forces relative jumps (which require 5 bytes).

```rust
pub fn replacement() -> i64 {
    37
}
```
The function that will replace `fun1`.

Finally we turn on size optimization in `Cargo.toml` to make sure that `fun1` and `fun2` are placed directly following each other without padding `nop`s:
```
[profile.dev]
opt-level = "s"
```

(Un-)Fortunately (depending on your point of view), rustc/llvm REALLY likes to inline (small) functions with size optimization even with the `#[inline(never)]` directive, but putting them in a separate crate seems to convince it not to do so.
