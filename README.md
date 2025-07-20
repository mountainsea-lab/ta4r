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




src/  
├── lib.rs                          # 库的根模块，重新导出公共 API  
├── bar/                            # Bar 模块目录  
│   ├── mod.rs                      # Bar 模块的入口文件  
│   ├── types.rs                    # Bar 和 BarSeries 的 trait 定义  
│   ├── base_bar.rs                 # BaseBar<T> 实现  
│   ├── base_bar_series.rs          # BaseBarSeries<T> 实现  
│   ├── builder/                    # Builder 相关实现  
│   │   ├── mod.rs                  # Builder 模块入口  
│   │   ├── types.rs                # BarBuilder 和 BarBuilderFactory trait  
│   │   ├── time_bar_builder.rs     # TimeBarBuilder<T> 实现  
│   │   ├── tick_bar_builder.rs     # TickBarBuilder<T> 实现  
│   │   ├── volume_bar_builder.rs   # VolumeBarBuilder<T> 实现  
│   │   ├── heikin_ashi_builder.rs  # HeikinAshiBarBuilder<T> 实现  
│   │   └── factory/                # Factory 实现  
│   │       ├── mod.rs              # Factory 模块入口  
│   │       ├── time_factory.rs     # TimeBarBuilderFactory  
│   │       └── heikin_ashi_factory.rs # HeikinAshiBarBuilderFactory  
│   ├── aggregator/                 # 聚合器实现  
│   │   ├── mod.rs                  # Aggregator 模块入口  
│   │   ├── types.rs                # BarAggregator trait 定义  
│   │   ├── duration_aggregator.rs  # DurationBarAggregator  
│   │   └── heikin_ashi_aggregator.rs # HeikinAshiBarAggregator  
│   └── series_builder.rs           # BaseBarSeriesBuilder<T> 实现  
├── num/                            # 数值系统模块  
│   ├── mod.rs                      # 数值模块入口  
│   ├── types.rs                    # TrNum 和 NumFactory trait 定义  
│   ├── error.rs                    # NumError 定义  
│   ├── factory/                    # NumFactory 实现  
│   │   ├── mod.rs                  # Factory 模块入口  
│   │   ├── double_factory.rs       # DoubleNumFactory (f64)  
│   │   ├── decimal_factory.rs      # DecimalNumFactory (Decimal)  
│   │   └── nan_factory.rs          # NaNFactory (NaN)  
│   └── impls/                      # TrNum 具体实现  
│       ├── mod.rs                  # 实现模块入口  
│       ├── double_num.rs           # f64 的 TrNum 实现  
│       ├── decimal_num.rs          # Decimal 的 TrNum 实现  
│       └── nan.rs                  # NaN 的 TrNum 实现  
└── utils/                          # 工具模块（如果需要）  
├── mod.rs  
└── constants.rs                # 常量定义  

模块导出结构
lib.rs
pub mod bar;  
pub mod num;

// 重新导出常用类型  
pub use bar::{Bar, BarSeries, BaseBar, BaseBarSeries};  
pub use num::{TrNum, NumFactory, NumError};
bar/mod.rs
pub mod types;  
pub mod base_bar;  
pub mod base_bar_series;  
pub mod builder;  
pub mod aggregator;  
pub mod series_builder;

// 重新导出公共 API  
pub use types::{Bar, BarSeries, BarBuilder, BarBuilderFactory, BarAggregator};  
pub use base_bar::BaseBar;  
pub use base_bar_series::BaseBarSeries;
num/mod.rs
pub mod types;  
pub mod error;  
pub mod factory;  
pub mod impls;

// 重新导出公共 API  
pub use types::{TrNum, NumFactory};  
pub use error::NumError;