// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.

// TODO: fix this broken import. Create a new library target in the `src` directory.
//   The library target should expose a public function named `hello_world` that takes no arguments
//   and returns nothing.
use packages::hello_world;

// 20240528: FIXME(liigo): 什么情况？use packages::*? 哪里来的，Cargo.toml?
// 经实测，lib.rs和main.rs并存时，lib.rs内的函数所属mod的确是Cargo.toml内指定的[package].name（即此处packages）。
// 官方答案的确可以通过编译。但我不知道他要考什么知识点，有点偏门了吧。
// 我的答案是，函数hello_world写在新文件packages.rs内，然后main.rs新增一行mod packages;。也编译通过了。mod基本用法呀。
// 这一课的标题是packages，貌似不是教mod用法:( see https://rust-exercises.com/05_ticket_v2/10_packages
// 我又实验了一下，main.rs和lib.rs并存虽然奇怪但的确可以编译，通过cargo -v build看到它两次调用rustc先编译了lib后编译了bin，
// 并且在编译bin时自动依赖了前面编译的lib。lib和bin的crate-name都一样是[package].name。自动生成的Cargo.toml内未明确指定crate-type。
//
// 我感觉，lib和bin并存的情况下，使用[[bin]]name="bin1"才是正途吧，对应的源文件是`src\bin\bin1.rs`或`src\bin\bin1\main.rs`。
// 那bin\bin1.rs怎么使用lib内函数呢，还是要`use pkgname::fnname`呀。回到src\main.rs也一样，它被默认视为一个bin（其name与lib同名）。
// 终于想通了。
// 一个package只能有一个[lib]，但可有多个[[bin]]，src\main.rs被默认视为其中一个[[bin]](无需定义)，默认与lib同名。
// [[bin]]内引用[lib]的常规方法就是：`use pkgname::*;`，其中pkgname是来自Cargo.toml里的[package]name="pkgname"。
// 该pkgname是crate的名字，不是mod的名字。

// This is the entrypoint of the binary.
fn main() {
    hello_world();
}
