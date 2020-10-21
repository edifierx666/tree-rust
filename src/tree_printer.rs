use crate::file_iterator::{FileItem, FileIterator};
use crate::CliConfig;

pub struct Summary {
  pub num_folders: usize,
  pub num_files: usize,
}
pub struct TreePrinter {
  pub config: CliConfig,
}
pub mod prefix {
  pub const HORZ: char = '─';
  pub const CROSS: char = '├';
  pub const VERT: char = '│';
  pub const LAST: char = '└';
}

impl TreePrinter {
  pub fn new(config: CliConfig) -> TreePrinter {
    TreePrinter { config }
  }
  pub fn iterator_folders(&self) {
    let mut summary = Summary {
      num_files: 0,
      num_folders: 0,
    };
    let mut helper = vec![];
    let mut iterator = self.get_iterator();
    iterator.next();
    for item in iterator {
      Self::helper(&mut helper, &item);
      Self::print_path(&helper, &item);
      if item.is_dir {
        summary.num_folders += 1;
      } else {
        summary.num_files += 1;
      }
    }
    Self::print_summary(&mut summary);
  }
  fn get_iterator(&self) -> FileIterator {
    FileIterator::new(self.config.clone())
  }
  fn print_summary(summary: &mut Summary) {
    println!(
      "{} 个文件夹, {} 个文件",
      summary.num_folders, summary.num_files
    );
  }
  fn print_path(helper: &Vec<bool>, item: &FileItem) {
    let len = helper.len();
    let mut s = String::new();
    let index = len - 1;
    for level in helper.iter().take(index) {
      if *level {
        s.push(prefix::VERT);
        s.push_str("   ");
      } else {
        s.push_str("    ");
      }
    }
    if let Some(last) = helper.last() {
      if *last {
        s.push(prefix::CROSS);
      } else {
        s.push(prefix::LAST);
      }
      s.push(prefix::HORZ);
      s.push(prefix::HORZ);
    }
    println!("{} {}", s, item.file_name);
  }
  fn helper(helper: &mut Vec<bool>, item: &FileItem) {
    while helper.len() > item.level {
      helper.pop();
    }
    if item.level > helper.len() {
      helper.push(!item.is_last);
    }
    let len = helper.len();
    if len > 0 {
      helper[len - 1] = !item.is_last;
    }
  }
}
