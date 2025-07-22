# English Vocabulary Memorization Aid 🎯
+ [![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://releases.rs/docs/1.70.0/)
+ [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> 考研英语词汇记忆助手 | 基于 Rust 开发的命令行工具

## ✨ 功能特性
- 📖 **词库解析**：自动读取 `example.txt` 中的词汇（格式：`单词::词性::释义`）
- 🎮 **交互学习**：按行号浏览单词，按任意键显示释义
- 📊 **进度统计**：显示已学习词汇量（共 XX 个单词）

## 🛠️ 安装与运行
### 前置要求
- Rust 1.70+  
+  ```bash
+  rustup update stable
+  ```

### 快速开始
-1.克隆仓库：git clone https://github.com/musicandrap/English-vocabulary-memorization-aid-for-postgraduate-entrance-examination.git
-2.切换目录：cd English-vocabulary-memorization-aid-for-postgraduate-entrance-examination/tools
-3.运行：cargo run --release
+ 1. 克隆仓库：
+   ```bash
+   git clone https://github.com/musicandrap/English-vocabulary-memorization-aid-for-postgraduate-entrance-examination.git
+   cd English-vocabulary-memorization-aid-for-postgraduate-entrance-examination/tools
+   ```
+ 2. 运行程序（推荐 release 模式）：
+   ```bash
+   cargo run --release
+   ```
+   *一键复制按钮会自动出现在代码块右上角*

+ ## 🖥️ 界面预览
+ ```plaintext
+ 开始单词背诵 (共 50 个单词)
+ 输入任意键查看答案，输入 q 退出
+ 
+ (1) abandon
+ [按回车显示]
+ v 放弃
+ ```
