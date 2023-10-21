# Brainfuck RS

BrainFuck interpreter written in Rust.

# Usage

Build the code locally.

```bash
 $ cargo build
```

Then run:

```bash
 $ ./target/debug/brainfuckrs filename
```

# Example

CLI example:

```bash
 $ ./target/debug/brainfuckrs ./examples/hw.bf
```

or code example

```rust
 let code: &str = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."

 read_compile_string(&code)
```
