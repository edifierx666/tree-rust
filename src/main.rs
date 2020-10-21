use clap::{App, Arg};
use std::env;
use std::path::PathBuf;
mod file_iterator;
mod tree_printer;
use tree_printer::TreePrinter;
#[derive(Clone)]
pub struct CliConfig {
  pub max_level: usize,
  pub start: String,
}

fn main() {
  // for (key, value) in env::vars_os() {
  //   println!("{:?}: {:?}", key, value);
  // }
  let start_path = path_to_str(env::current_dir().expect("目录解析错误"));
  // env! 读取环境变量
  // App 创建一个cli
  // Arg 创建一个参数
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .args(&[
      Arg::with_name("DIR")
        .index(1)
        .help("tree目录")
        .default_value(&start_path), // 默认是当前目录
      Arg::with_name("Level") // 目录深度
        .short("l") // -l
        .long("level") // --level
        .help("目录深度")
        .takes_value(true), // 需要给一个值
    ])
    .get_matches();
  let max_level = if let Some(max_level) = matches.value_of("Level") {
    to_int(max_level).expect("level转换int失败")
  } else {
    usize::max_value()
  };
  let start = matches.value_of("DIR").expect("获取目录错误").to_string();
  let cli_config = CliConfig { max_level, start };
  let printer = TreePrinter::new(cli_config);
  printer.iterator_folders();
}

fn to_int(v: &str) -> Result<usize, String> {
  v.parse().map_err(|e| format!("{}转整型失败:{}", v, e))
}
fn path_to_str(p: PathBuf) -> String {
  p.to_str().unwrap_or(".").to_string()
}
