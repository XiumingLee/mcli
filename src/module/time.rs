use crate::module::Command;
use clap::{App, ArgMatches, Arg};
use time::{OffsetDateTime};
use time::{format_description};
use time::format_description::FormatItem;

/// 时间相关的命令    https://docs.rs/time/0.3.2/time/
/// 1、时间戳转yyyy-mm-dd HH:MM:SS
/// 2、yyyy-MM-dd HH:mm:SS 转时间戳
/// 3、当前时间转时间戳
pub struct Time {}

const NOW: &str = "now";
/// 单位  s/ms
const UNIT: &str = "unit";
/// 时间戳
const TIMESTAMP: &str = "timestamp";
/// 格式化时间字符串
const TIME_STR: &str = "string";
/// 时间格式化
const  DATE_TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

impl Command for Time {
    fn get_app<'a>() -> App<'a> {
        App::new("time")
            .about("日期时间操作子命令")
            .arg(
                Arg::new(NOW)
                    .long(NOW)
                    .short('n')
                    .about("输出当前时间的时间戳")
                    .required(false)
            )
            .arg(
                Arg::new(UNIT)
                    .long(UNIT)
                    .about("时间戳的单位 s/ms")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::new(TIMESTAMP)
                    .long(TIMESTAMP)
                    .short('t')
                    .about("时间戳 此参数和 -s 二者只能选其一 ")
                    .takes_value(true)
                    .required(false)
            )
            .arg(
                Arg::new(TIME_STR)
                    .long(TIME_STR)
                    .short('s')
                    .about("格式化时间字符串,只支出yyyy-MM-dd hh:mm:ss格式，此参数和 -t 二者只能选其一")
                    .takes_value(true)
                    .required(false)
            )
    }

    fn get_fn() -> fn(&ArgMatches) -> Result<Vec<String>, String> {
        /// 具体的命令操作
        fn f(matches: &ArgMatches) -> Result<Vec<String>, String> {
            let now = matches.is_present(NOW);
            let timestamp = matches.is_present(TIMESTAMP);
            let time_str = matches.is_present(TIME_STR);
            if now {
                // 当前时间时间戳
                print_current_timestamp(matches)
            } else if timestamp {
                // 时间戳转格式化字符串
                timestamp_to_str(matches)
            } else if time_str {
                // 格式化字符串转时间戳
                str_to_timestamp(matches)
            } else {
                Err("命令输出错误！".to_string())
            }
        }
        f
    }
}

/// 打印当前时间戳
fn print_current_timestamp(matches: &ArgMatches) -> Result<Vec<String>, String> {
    let exist_unit = matches.is_present(UNIT);
    if !exist_unit {
        return Err("请添加单位参数 --unit s/ms！".to_string());
    }

    let unit = matches.value_of(UNIT).unwrap();
    if "s".eq(unit) {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        Ok(vec!["时间单位为s,当前时间戳为：".to_owned() + timestamp.to_string().as_str()])
    } else if "ms".eq(unit) {
        // 秒，毫秒，微秒，纳秒   所以纳秒除以1000000就是毫秒的值
        let timestamp = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000;
        Ok(vec!["时间单位为ms，当前时间戳为：".to_owned() + timestamp.to_string().as_str()])
    } else {
        Err("--unit 时间单位不正确！".to_string())
    }
}

fn timestamp_to_str(matches: &ArgMatches) -> Result<Vec<String>, String> {
    let  format: Vec<FormatItem>  = format_description::parse(DATE_TIME_FORMAT).unwrap();
    // 此处涉及到了强制类型转化
    let timestamp:i128 = matches.value_of(TIMESTAMP).unwrap().parse::<i128>().unwrap();
    let unit = matches.value_of(UNIT).unwrap();
    if "s".eq(unit) {
        let timestamp:i64 = timestamp as i64;
        let date_time_s = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
        let result = date_time_s.format(&format).unwrap();
        Ok(vec!["格式化的时间为：".to_owned() + result.as_str()])
    } else if "ms".eq(unit) {
        // 秒，毫秒，微秒，纳秒   所以纳秒除以1000000就是毫秒的值
        let timestamp = timestamp * 1_000_000;
        let date_time_ms = OffsetDateTime::from_unix_timestamp_nanos(timestamp).unwrap();
        let result = date_time_ms.format(&format).unwrap();
        Ok(vec!["格式化的时间为：".to_owned() + result.as_str()])
    } else {
        Err("--unit 时间单位不正确！".to_string())
    }
}

fn str_to_timestamp(matches: &ArgMatches) -> Result<Vec<String>, String> {

    Err("命令行参数不正确！".to_string())
}

#[test]
fn test_time() {
    let date_time:i128 = 1631524303317000000;
    let offset_date_time = OffsetDateTime::from_unix_timestamp_nanos(date_time).unwrap();
    let format = format_description::parse(DATE_TIME_FORMAT).unwrap();
    let result = offset_date_time.format(&format).unwrap();
    println!("{}",result)
    // format_description::parse()
    // offset_date_time.format()
    // offset_date_time.fmt()
    // offset_date_time
    // let x = offset_date_time.format(format_description::parse(DATE_TIME_FORMAT)?)?;
    // let result = offset_date_time.fmt(format_description::parse(DATE_TIME_FORMAT)).unwrap();
    // OffsetDateTime::f
    // println!("测试一下！{}",x);
}
