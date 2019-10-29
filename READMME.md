# Rust Crash

- `rustc hello.rs` 直接编译源码
- `cargo init` 初始化项目
- `cargo run` 编译测试版文件
- `cargo build --release` 编译正式版执行文件
- 引入模块

```rust
// print.rs
pub fn run() {
  println!("Hello from the print.rs file");
}

// main.rs
mod print;

fn main() {
    print::run();
}
```
