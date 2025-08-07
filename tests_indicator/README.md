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