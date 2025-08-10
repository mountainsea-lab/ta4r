✅ 命令建议大全
目标	命令	说明
运行全部测试	cargo test -p tests_indicator	运行 tests/ 下所有测试文件
运行某个 crate 级测试文件	cargo test --test indicator	运行 tests/indicator.rs（即该 crate）
运行某个子模块测试函数	cargo test --test indicator test_sma	运行 test_sma 函数（或模块中包含 sma 的测试名）
运行特定策略测试	cargo test --test strategy test_macd	运行 strategy crate 中所有包含 test_macd 的函数
运行包含关键词的测试函数	cargo test sma	全局模糊搜索名字含 sma 的测试函数
精确匹配某个测试函数	cargo test test_sma_exact_name	Rust 1.61+ 支持精确匹配函数名
显示测试输出	cargo test -- --nocapture	默认测试中 println! 不显示，加上此参数即可
并发数限制（调试）	cargo test -- --test-threads=1	让测试串行执行，方便调试
单线程 + 显示输出	cargo test -- --test-threads=1 --nocapture	最适合定位某个 test 输出

线上场景下：BarSeries + CachedIndicator 数据驱动流程
1. 初始化阶段
   从历史数据（数据库、API）加载最近 N 根 K 线（例如 1000 根）

创建一个 固定长度（max bars = 1000）的 BarSeries

Rust：内部可能是 VecDeque<BaseBar<T>>

保证新数据 push 时旧数据会自动 pop

用这个 BarSeries 创建各种 Indicator（包一层 CachedIndicator）

第一次计算：如果你要画全量图，就会调用所有 index 的 get_value(i)，缓存全部填满

2. 增量更新阶段（实时推送/订阅事件）
   新数据到来（WebSocket tick 或定时生成 K 线）：

如果是已存在的最后一根 bar（当前周期未结束），则更新它（OHLCV 变化）

如果是新 bar（周期结束），则 push 新的 bar，如果超出 1000 根，就自动 pop 最旧的 bar

BarSeries 变化后，对应的 CachedIndicator：

如果是 最后一根 bar 发生变化 → 清空 cache[last_index]

如果是 push 新 bar → cache.push(None)（Java/Rust 都是这样）

调用 indicator.get_value(last_index)：

如果缓存是 None → 计算并存入缓存

如果有值 → 直接返回（无需重新计算历史）

其他 index 的缓存值依然有效，不会重算

3. 滑动窗口（固定长度 1000 根）的影响
   当 push 新 bar 时，最旧的那根会被 pop

缓存同步移除最旧的值，index 对应的值和 BarSeries 索引保持一致

因为 CachedIndicator 是按 index 缓存的，所以这个操作必须同步进行，否则会错位

4. 流程总结
   perl
   复制
   编辑
   历史数据 → 初始化 BarSeries(1000) → 初始化指标缓存

↓ （实时推送）

新 tick / trade → 更新最后一根 bar 或 push 新 bar
→ 更新指标缓存 (清空最后一根 or push None)
→ get_value(last_index) 只计算最后一个值
→ 其他缓存保持不变
✅ 你的描述里唯一缺的点是：

最后一根 bar 被更新时，需要清空缓存对应位置，否则会用旧值

固定长度滑动窗口要同步移除缓存最旧的元素，否则 index 对应不上

初始化阶段如果你全量画图，第一次调用会触发全量计算（之后才是增量）