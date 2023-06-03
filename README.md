# `cli`: 一个`rust`由编写的脚手架

> 支持通过模板创建项目

## 项目结构

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
├── exec # 命令模块，负责执行命令
│   ├── Cargo.toml
│   └── src
│       ├── git.rs # git 相关命令
│       ├── lib.rs
│       └── utils.rs # 工具类
├── handler # 处理脚手架命令
│   ├── Cargo.toml
│   └── src
│       ├── create.rs # 创建项目命令
│       └── lib.rs
├── src
│   └── main.rs # 入口文件
```
## 使用方式
可以通过`cli -h`查看使用方式，当前只实现了一种命令`create`
```bash
Usage: cli [COMMAND]

Commands:
  create  创建项目
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 功能计划
- [x] 从服务端拉取模板
- [x] 创建项目 
- [x] 集成`CI/CD`