# Hecto
A simple Nano editor using Rust

### 项目介绍
参考 https://github.com/pflenker/hecto-tutorial 完成的类Nano文本编辑器项目\
`document.rs` 封装所有与文件交互的逻辑，内部数据结构为rows，以及相应行操作\
`row.rs` 封装行数据结构，对行内部文本的操作以及渲染\
`terminal.rs` 封装控制台操作\
`editor.rs` 封装nano-like的指令逻辑，状态栏、行号显示，整体文本显示，管理document数据结构。对外暴露编辑器run接口

### 使用指南
所有基本操作均为自然逻辑（why Nano!）\
状态栏第1行：依次显示：当前文件 行号\
状态栏第2行：命令prompt提示

### 编译
`cargo build --release`
