## **📈 ta4r — Technical Analysis for Rust**
`
ta4r is an open-source Rust library for technical analysis of financial time series.
It provides a modular, composable framework to build, test, and execute trading strategies with idiomatic Rust design.
`
---
### **✨ Features**
 * ✅ Core technical indicators: SMA, EMA, RSI, MACD, Bollinger Bands, etc.

 * 📐 Strategy construction with rules and logical combinators

 * 🕰️ Time series primitives: Bar, BarSeries

 * 🧪 Built-in backtesting engine

 * 🦀 Fully written in Rust — fast, safe, and no runtime GC
---
### **🧠 Philosophy**
`
 Inspired by the popular ta4j Java library,
ta4r brings a modern, performance-oriented approach to technical analysis in Rust.
Instead of relying on inheritance or OOP hierarchies, ta4r uses Rust traits and composable functions to provide maximum flexibility with zero-cost abstractions.
`
---
### **📦 Installation**
```text
    # Cargo.toml
    [dependencies]
    ta4r = "0.1"
```
##### Or run:
```shell
  cargo add ta4r
```
---
### **🙏 Acknowledgement**
`
          This project is heavily inspired by ta4j — Technical Analysis for Java,
    an open-source Java library developed by Marc de Verdelhan and contributors.
    We sincerely thank the authors of ta4j for their clean design, rich indicator implementations, and comprehensive strategy modeling framework
    While ta4r reimplements these ideas with idiomatic Rust and a performance-first architecture,
    its conceptual foundations are built upon ta4j. Without their contributions, this project wouldn't exist.
    ta4j is licensed under the MIT License.
    ta4r also follows the MIT License —
    because we believe that open source makes the world greater. 🌍
`
>   ta4r 致力于构建一个自由、可组合、高性能的技术分析引擎，感谢开源精神让这一切成为可能。




