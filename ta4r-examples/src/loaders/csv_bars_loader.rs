// use csv::ReaderBuilder;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::PathBuf;
// use ta4r::bar::base_bar::BaseBar;
// use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
// use ta4r::num::decimal_num::DecimalNum;
// use ta4r::num::decimal_num_factory::DecimalNumFactory;
// use ta4r::num::NumFactory;
// use time::{Duration, OffsetDateTime};
//
// /// CSV 格式: timestamp(ms), open, high, low, close, volume
// pub struct CsvBarsLoader;
//
// impl CsvBarsLoader {
//     /// 自动生成 assets 下文件路径
//     fn get_asset_path(filename: &str) -> PathBuf {
//         let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         path.push("assets");
//         path.push(filename);
//         path
//     }
//
//     /// 加载 CSV 文件，支持任意周期 duration
//     pub fn load_csv_series(
//         filename: &str,
//         period: Duration,
//     ) -> Result<BaseBarSeriesBuilder<DecimalNum>, String> {
//         let path = Self::get_asset_path(filename);
//         let file = File::open(&path)
//             .map_err(|e| format!("Failed to open file {}: {}", path.display(), e))?;
//         let reader = BufReader::new(file);
//
//         let mut csv_reader = ReaderBuilder::new()
//             .has_headers(true)
//             .from_reader(reader);
//
//         let mut builder = BaseBarSeriesBuilder::<DecimalNum>::new()
//             .with_name(filename.to_string());
//
//         let factory = DecimalNumFactory::instance();
//
//         for result in csv_reader.records() {
//             let record = result.map_err(|e| format!("CSV parse error: {}", e))?;
//
//             let timestamp_ms: i64 = record.get(0)
//                 .ok_or("Missing timestamp")?
//                 .parse()
//                 .map_err(|e| format!("Invalid timestamp: {}: {}", record.get(0).unwrap_or(""), e))?;
//
//             let dt = OffsetDateTime::from_unix_timestamp(timestamp_ms / 1000)
//                 .map_err(|e| format!("Invalid timestamp: {}: {}", timestamp_ms, e))?
//                 + Duration::milliseconds(timestamp_ms % 1000);
//
//             let open: DecimalNum = factory.num_of_f64(
//                 record.get(1)
//                     .ok_or("Missing open")?
//                     .parse::<f64>()
//                     .map_err(|e| format!("Invalid open: {}", e))?,
//             );
//             let high: DecimalNum = factory.num_of_f64(
//                 record.get(2)
//                     .ok_or("Missing high")?
//                     .parse::<f64>()
//                     .map_err(|e| format!("Invalid high: {}", e))?,
//             );
//             let low: DecimalNum = factory.num_of_f64(
//                 record.get(3)
//                     .ok_or("Missing low")?
//                     .parse::<f64>()
//                     .map_err(|e| format!("Invalid low: {}", e))?,
//             );
//             let close: DecimalNum = factory.num_of_f64(
//                 record.get(4)
//                     .ok_or("Missing close")?
//                     .parse::<f64>()
//                     .map_err(|e| format!("Invalid close: {}", e))?,
//             );
//             let volume: DecimalNum = factory.num_of_f64(
//                 record.get(5)
//                     .ok_or("Missing volume")?
//                     .parse::<f64>()
//                     .map_err(|e| format!("Invalid volume: {}", e))?,
//             );
//
//             let bar = BaseBar::<DecimalNum> {
//                 time_period: period,
//                 begin_time: dt - period,
//                 end_time: dt,
//                 open_price: Some(open),
//                 high_price: Some(high),
//                 low_price: Some(low),
//                 close_price: Some(close),
//                 volume,
//                 amount: None,
//                 trades: 0,
//             };
//
//             // 高性能直接追加，不 clone Vec
//             builder.bars.push(bar);
//         }
//
//         Ok(builder)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ta4r::bar::types::{BarSeries, BarSeriesBuilder};
//     use time::Duration;
//
//     #[test]
//     fn test_load_apple_inc_csv() -> Result<(), String> {
//         // CSV 文件放在 assets/ 下
//         let builder = CsvBarsLoader::load_csv_series(
//             "appleinc_bars_from_20130101_usd.csv",
//             Duration::minutes(5),
//         )?;
//         let series = builder.build().map_err(|e| format!("Build error: {:?}", e))?;
//
//         println!("Series: {}", series.get_name());
//         println!("Number of bars: {}", series.get_bar_count());
//
//         if let Some(bar) = series.get_bar(0) {
//             println!("\tOpen: {}", bar.open_price.as_ref().map_or("-", |v| &v.to_string()));
//             println!("\tHigh: {}", bar.high_price.as_ref().map_or("-", |v| &v.to_string()));
//             println!("\tLow: {}", bar.low_price.as_ref().map_or("-", |v| &v.to_string()));
//             println!("\tClose: {}", bar.close_price.as_ref().map_or("-", |v| &v.to_string()));
//             println!("\tVolume: {}", bar.volume);
//         }
//
//         Ok(())
//     }
// }

use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use time::{Duration, OffsetDateTime};
use time_macros::format_description;
use ta4r::bar::base_bar::BaseBar;
use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use ta4r::num::decimal_num::DecimalNum;
use ta4r::num::decimal_num_factory::DecimalNumFactory;
use ta4r::num::NumFactory;

/// CSV 格式支持两种：
/// 1. timestamp(ms), open, high, low, close, volume
/// 2. yyyy-MM-dd, open, high, low, close, volume
pub struct CsvBarsLoader;

impl CsvBarsLoader {
    /// 自动生成 assets 下文件路径
    fn get_asset_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("assets");
        path.push(filename);
        path
    }

    /// 尝试解析 timestamp 或日期字符串
    fn parse_datetime(s: &str) -> Result<OffsetDateTime, String> {
        // 尝试先解析为毫秒时间戳
        if let Ok(ms) = s.parse::<i64>() {
            let dt = OffsetDateTime::from_unix_timestamp(ms / 1000)
                .map_err(|e| format!("Invalid timestamp {}: {}", ms, e))?
                + Duration::milliseconds(ms % 1000);
            return Ok(dt);
        }

        // 尝试解析为日期字符串 "yyyy-mm-dd"
        let format = format_description!("[year]-[month]-[day]");
        let dt = OffsetDateTime::parse(s, &format)
            .map_err(|e| format!("Invalid date string '{}': {}", s, e))?;
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

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);

        let mut builder = BaseBarSeriesBuilder::<DecimalNum>::new()
            .with_name(filename.to_string());

        let factory = DecimalNumFactory::instance();

        for result in csv_reader.records() {
            let record = result.map_err(|e| format!("CSV parse error: {}", e))?;
            let ts_str = record.get(0).ok_or("Missing timestamp")?;
            let dt = Self::parse_datetime(ts_str)?;

            let open: DecimalNum = factory.num_of_f64(
                record.get(1)
                    .ok_or("Missing open")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid open: {}", e))?,
            );
            let high: DecimalNum = factory.num_of_f64(
                record.get(2)
                    .ok_or("Missing high")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid high: {}", e))?,
            );
            let low: DecimalNum = factory.num_of_f64(
                record.get(3)
                    .ok_or("Missing low")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid low: {}", e))?,
            );
            let close: DecimalNum = factory.num_of_f64(
                record.get(4)
                    .ok_or("Missing close")?
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid close: {}", e))?,
            );
            let volume: DecimalNum = factory.num_of_f64(
                record.get(5)
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

    #[test]
    fn test_load_apple_inc_csv() -> Result<(), String> {
        let builder = CsvBarsLoader::load_csv_series(
            "appleinc_bars_from_20130101_usd.csv",
            Duration::minutes(5),
        )?;
        let series = builder.build().map_err(|e| format!("Build error: {:?}", e))?;

        println!("Series: {}", series.get_name());
        println!("Number of bars: {}", series.get_bar_count());

        if let Some(bar) = series.get_bar(0) {
            println!("\tOpen: {}", bar.open_price.as_ref().map_or("-", |v| &v.to_string()));
            println!("\tHigh: {}", bar.high_price.as_ref().map_or("-", |v| &v.to_string()));
            println!("\tLow: {}", bar.low_price.as_ref().map_or("-", |v| &v.to_string()));
            println!("\tClose: {}", bar.close_price.as_ref().map_or("-", |v| &v.to_string()));
            println!("\tVolume: {}", bar.volume);
        }

        Ok(())
    }
}
