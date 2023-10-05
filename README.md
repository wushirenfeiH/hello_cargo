<!--
 * @Description: ***页面
 * @Date: 2023-10-03 17:10:00
 * @Author: hkr
 * @LastEditors: hkr
-->

# 简介

## 为什么要用 Rust？

- Rust 是一种令人兴奋的新编程语言，它可以让每个人编写可靠且高效的软件。
- 它可以用来替换 C/C++，Rust 和它们具有同样的性能，但是很多常见的 bug 在编译时就可以被消灭
- Rust 是一种通用的编程语言，但是它更善于以下场景:
  - 需要运行时的速度
  - 需要内存安全
  - 更好的利用多处理器

## 与其他语言比较

- C/C++ 性能非常好，但类型系统和内存都不太安全
- Java/C#，拥有 GC，能保证内存安全，也有很多优秀特性，但是性能不行。
- Rust:
  - 安全
  - 无需 GC
  - 易于维护、调试，代码安全高效

## Rust 特别擅长的领域

- 高性能 Web Service。WebAssembly
- 命令行工具
- 网络编程
- 嵌入式设备

# 安装

cargo 脚手架???
初始化一个 git 仓库

# Cargo.toml - 类似 package.json

## https://crates.io/crates

- 怎么登录(github)
- 怎么发包

## package --- 包的配置项

- name
- versopm
- edition

## dependencies --- 项目的依赖 webpack -S ？？？

## cargo run - 编译代码 + 执行结果

- 如果之前有编译过，并且没改过源代码，就直接运行二进制文件

## cargo check

- 检查代码, 确保能通过编译，不产生任何可执行文件
- 执行要比 cargo build 快很多
- - 编写代码的时候可以连续反复的使用 cargo check 检查代码，提高效率

## cargo build --release

- 编译时会进行优化
- - 代码会运行得更快, 但是编译时间更长
- 会在 target/release 而不是 target/debug 生成可执行文件

## cargo build 跟 cargo build --release 区别

- 开发
- 发布包
