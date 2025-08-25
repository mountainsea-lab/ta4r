use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use ta4r::bar::base_bar::BaseBar;
use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use ta4r::num::NumFactory;
use ta4r::num::decimal_num::DecimalNum;
use ta4r::num::decimal_num_factory::DecimalNumFactory;
use time::{Date, Duration, OffsetDateTime};
use time_macros::format_description;

/// CSV 格式支持两种：
/// 1. timestamp(ms), open, high, low, close, volume
/// 2. yyyy-MM-dd, open, high, low, close, volume
pub struct CsvBarsLoader;

impl CsvBarsLoader {
    /// 智能解析 CSV 文件路径
    /// - 优先使用当前 crate 根目录 assets/
    /// - 如果不存在，则尝试 workspace 根目录下 ta4r-examples/assets/
    pub fn get_asset_path(filename: &str) -> PathBuf {
        // 当前 crate 根目录
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // 1️⃣ 尝试 crate 下 assets/
        let candidate = crate_root.join("assets").join(filename);
        if candidate.exists() {
            return candidate;
        }

        // 2️⃣ 尝试 workspace 根目录
        // 这里假设 workspace 结构是：workspace_root/ta4r-examples/assets/...
        let mut workspace_root = crate_root.as_path();
        while let Some(parent) = workspace_root.parent() {
            workspace_root = parent;
            if workspace_root.join("ta4r-examples").exists() {
                break;
            }
        }

        let candidate = workspace_root.join("ta4r-examples/assets").join(filename);
        if candidate.exists() {
            return candidate;
        }

        // 最后返回默认路径，让 File::open 报错
        crate_root.join("assets").join(filename)
    }

    /// 尝试解析 timestamp 或日期字符串
    fn parse_datetime(s: &str) -> Result<OffsetDateTime, String> {
        // 优先解析为毫秒时间戳
        if let Ok(ms) = s.parse::<i64>() {
            let dt = OffsetDateTime::from_unix_timestamp(ms / 1000)
                .map_err(|e| format!("Invalid timestamp {}: {}", ms, e))?
                + Duration::milliseconds(ms % 1000);
            return Ok(dt);
        }

        // 解析为日期字符串 "yyyy-mm-dd"
        let date_format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(s, &date_format)
            .map_err(|e| format!("Invalid date string '{}': {}", s, e))?;
        let dt: OffsetDateTime = date.midnight().assume_utc();

        Ok(dt)
    }

    /// 加载 CSV 文件，支持任意周期 duration
    pub fn load_csv_series(
        filename: &str,
        period: Duration,
    ) -> Result<BaseBarSeriesBuilder<DecimalNum>, String> {
        let path = Self::get_asset_path(filename);
        let file = File::open(&path)
            .map_err(|e| format!("Failed to open file {}: {}", path.display(), e))?;
        let reader = BufReader::new(file);

        let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        let mut builder = BaseBarSeriesBuilder::<DecimalNum>::new().with_name(filename.to_string());

        let factory = DecimalNumFactory::instance();

        for result in csv_reader.records() {
            let record = result.map_err(|e| format!("CSV parse error: {}", e))?;
            let ts_str = record.get(0).ok_or("Missing timestamp")?;
            let dt = Self::parse_datetime(ts_str)?;

            let open: DecimalNum = factory.num_of_f64(
                record
                    .get(1)
                    .ok_or("Missing open")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid open: {}", e))?,
            );
            let high: DecimalNum = factory.num_of_f64(
                record
                    .get(2)
                    .ok_or("Missing high")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid high: {}", e))?,
            );
            let low: DecimalNum = factory.num_of_f64(
                record
                    .get(3)
                    .ok_or("Missing low")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid low: {}", e))?,
            );
            let close: DecimalNum = factory.num_of_f64(
                record
                    .get(4)
                    .ok_or("Missing close")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid close: {}", e))?,
            );
            let volume: DecimalNum = factory.num_of_f64(
                record
                    .get(5)
                    .ok_or("Missing volume")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid volume: {}", e))?,
            );

            let bar = BaseBar::<DecimalNum> {
                time_period: period,
                begin_time: dt - period,
                end_time: dt,
                open_price: Some(open),
                high_price: Some(high),
                low_price: Some(low),
                close_price: Some(close),
                volume,
                amount: None,
                trades: 0,
            };

            // 高性能直接追加，不 clone Vec
            builder.bars.push(bar);
        }

        Ok(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ta4r::bar::types::{BarSeries, BarSeriesBuilder};
    use time::Duration;

    /// 安全打印 Option<DecimalNum>
    fn fmt_decimal_option(opt: &Option<DecimalNum>) -> String {
        match opt {
            Some(v) => v.to_string(),
            None => "-".to_string(),
        }
    }

    #[test]
    fn test_load_apple_inc_csv() -> Result<(), String> {
        let builder = CsvBarsLoader::load_csv_series(
            "appleinc_bars_from_20130101_usd.csv",
            Duration::days(1),
        )?;
        let series = builder
            .build()
            .map_err(|e| format!("Build error: {:?}", e))?;

        println!("Series: {}", series.get_name());
        println!("Number of bars: {}", series.get_bar_count());

        if let Some(bar) = series.get_bar(0) {
            println!("\tOpen: {}", fmt_decimal_option(&bar.open_price));
            println!("\tHigh: {}", fmt_decimal_option(&bar.high_price));
            println!("\tLow: {}", fmt_decimal_option(&bar.low_price));
            println!("\tClose: {}", fmt_decimal_option(&bar.close_price));
            println!("\tVolume: {}", bar.volume);
        }

        Ok(())
    }

    #[test]
    fn test_load_xrp_csv() -> Result<(), String> {
        let builder = CsvBarsLoader::load_csv_series(
            "XRPUSD_251226-5m-2025-08-22.csv",
            Duration::minutes(5),
        )?;
        let series = builder
            .build()
            .map_err(|e| format!("Build error: {:?}", e))?;

        println!("Series: {}", series.get_name());
        println!("Number of bars: {}", series.get_bar_count());

        if let Some(bar) = series.get_bar(0) {
            println!("\tOpen: {}", fmt_decimal_option(&bar.open_price));
            println!("\tHigh: {}", fmt_decimal_option(&bar.high_price));
            println!("\tLow: {}", fmt_decimal_option(&bar.low_price));
            println!("\tClose: {}", fmt_decimal_option(&bar.close_price));
            println!("\tVolume: {}", bar.volume);
        }

        Ok(())
    }
}
