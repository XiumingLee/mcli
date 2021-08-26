mod file;

use clap::{App, ArgMatches};
use crate::module::file::File;
use std::collections::HashMap;
use std::io;


#[derive(Clone)]
pub struct Module<'a> {
    pub app: App<'a>,
    pub f: fn(&ArgMatches) -> Result<Vec<String>, String>,
    pub desc: String,
}

/// 模块管理器
pub struct ModuleManager<'a> {
   pub modules: HashMap<String, Module<'a>>,
}

impl<'a> ModuleManager<'a> {
    pub fn new() -> Self {
        let mut mm = Self {
            modules: HashMap::new(),
        };
        mm.register(File::module());
        mm
    }

    /// 执行匹配的模块方法
    pub fn run(&self, name: &str, matches: &ArgMatches) {
        let result = match name {
            _ => (self.modules.get(name).expect("subcommand must exist!").f)(matches),
        };

        match result {
            Ok(result) => result.iter().for_each(|x| println!("{}", x)),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn register(&mut self, module: Module<'a>) {
        self.modules.insert(module.app.get_name().to_string(), module);
    }
}


pub trait Command {
    fn module<'a>() -> Module<'a> {
        Module {
            app: Self::get_app(),
            f: Self::get_fn(),
            desc: Self::get_desc(),
        }
    }

    fn get_app<'a>() -> App<'a>;
    fn get_fn() -> fn(&ArgMatches) -> Result<Vec<String>, String>;
    fn get_desc() -> String;

    /// 用于确定是否继续执行
    fn confirm() -> bool {
        println!("确定执行以上操作吗？(y/n)");
        let mut yn = String::new();
        io::stdin().read_line(&mut yn)
            .expect("请输入y或者n。");

        match yn.to_lowercase().trim() {
            "y" => true,
            "n" => false,
            _ => Self::confirm(),
        }
    }
}
