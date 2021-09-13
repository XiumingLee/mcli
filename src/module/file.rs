use clap::{ArgMatches, App, Arg};
use crate::module::{Command};
use std::fs;
use std::io;
use std::path;
use std::path::Path;

/// file (-rm(--remove)/-rn(--rename)) [-b(--batch)]
/// mcli file --remove --path "./" -c "(2)"   // 删除当前文件夹下，文件名包含(2)的文件
pub struct File {}

const REMOVE: &str = "remove";
const PATH: &str = "path";
const CONDITION: &str = "condition";

impl Command for File {
    fn get_app<'a>() -> App<'a> {
        App::new("file")
            .about("file 文件操作子命令")
            .arg(
                Arg::new(REMOVE)
                    .long(REMOVE)
                    .about("删除满足条件的文件;mcli file --remove 后面跟参数")
                    .required(false)
            )
            .arg(
                Arg::new(PATH)
                    .long(PATH)
                    .about("要操作的文件路径")
                    .takes_value(true)  // 获取参数值，要不 拿不到后面跟的值
                    .required(true)
            )
            .arg(
                Arg::new(CONDITION)
                    .long(CONDITION)
                    .short('c')
                    .about("满足的条件")
                    .takes_value(true)
                    .required(false)
            )
    }

    fn get_fn() -> fn(&ArgMatches) -> Result<Vec<String>, String> {
        /// 具体的命令操作
        fn f(matches: &ArgMatches) -> Result<Vec<String>, String> {
            let remove = matches.is_present("remove");

            if remove {
                // 文件删除操作
                remove_file_by_condition(matches)
            } else {
                Err("命令输出错误！".to_string())
            }
        }
        f
    }
}

fn remove_file_by_condition(matches: &ArgMatches) -> Result<Vec<String>, String> {
    let path = path::Path::new(matches.value_of(PATH).unwrap());
    if !path.exists() {
        return Err("path 路径不存在！".to_string());
    }

    let condition_option = matches.value_of(CONDITION);

    println!("执行目录为：{}",path.to_str().unwrap());
    let result = match condition_option {
        Some(condition) => {
            println!("执行条件为：{}",condition);
            if File::confirm() {
                remove_all_sub_files_by_condition(path, condition).unwrap();
                return Ok(vec!["完成！".to_string()]);
            }
            return Ok(vec!["取消！".to_string()]);
        }
        None => {
            // 删除输入的整个文件夹或文件
            if File::confirm() {
                return if path.is_dir() {
                    match fs::remove_dir_all(path) {
                        Ok(_) => {
                            Ok(vec!["删除文件夹：".to_owned() + path.to_str().unwrap()])
                        }
                        Err(e) => {
                            Err(e.to_string())
                        }
                    }
                } else {
                    remove_file(path)
                };
            }
            Ok(vec!["".to_string()])
        }
    };
    result
}

/// 删除文件
fn remove_file(path: &Path) -> Result<Vec<String>, String> {
    match fs::remove_file(path) {
        Ok(_) => {
            Ok(vec!["删除文件：".to_owned() + path.to_str().unwrap()])
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}


/// 移除所有符合条件的子文件
fn remove_all_sub_files_by_condition(path: &Path, condition: &str) -> io::Result<()> {
    if path.is_dir() {
        let read_dir = path.read_dir().unwrap();
        for dir in read_dir {
            let path_buf = dir.unwrap().path();
            if path_buf.is_dir() {
                remove_all_sub_files_by_condition(path_buf.as_path(), condition).unwrap();
            } else {
                // 文件
                if path_buf.file_name().unwrap().to_str().unwrap().contains(condition) {
                    remove_file(path_buf.as_path()).unwrap();
                }
            }
        }
    } else {
        // 文件
        if path.file_name().unwrap().to_str().unwrap().contains(condition) {
            remove_file(path).unwrap();
        }
    }
    Ok(())
}


