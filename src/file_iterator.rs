use crate::CliConfig;
use std::collections::VecDeque;
use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

pub struct FileIterator {
  pub queue: VecDeque<FileItem>,
  pub config: CliConfig,
}

impl FileIterator {
  fn push_dir(&mut self, item: &mut FileItem) {
    let sort_dirs = Self::sort_dirs(item);
    let filter_dirs = Self::filter_dirs(sort_dirs);
    let mut file_items: Vec<_> = filter_dirs
      .iter()
      .map(|dir| {
        let mut item1 = FileItem::new(dir.path());
        item1.set_level(item.level + 1);
        item1
      })
      .collect();
    if let Some(item) = file_items.first_mut() {
      item.set_is_last(true);
    };
    for file_item in file_items {
      self.queue.push_front(file_item);
    }
  }
  fn sort_dirs(item: &mut FileItem) -> Vec<DirEntry> {
    item
      .path
      .read_dir()
      .map(|read_dir| {
        let mut dirs: Vec<DirEntry> = read_dir.filter_map(Result::ok).collect();
        if dirs.len() > 0 {
          item.set_is_empty(false);
        }
        dirs.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        dirs
      })
      .expect("dirs排序错误")
  }
  fn filter_dirs(dirs: Vec<DirEntry>) -> Vec<DirEntry> {
    dirs
  }
}

impl FileIterator {
  pub fn new(config: CliConfig) -> FileIterator {
    let mut queue = VecDeque::new();
    queue.push_back(FileItem::new(PathBuf::from(&config.start)));
    FileIterator { queue, config }
  }
}
impl Iterator for FileIterator {
  type Item = FileItem;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(mut item) = self.queue.pop_front() {
      if item.is_dir && item.level < self.config.max_level {
        self.push_dir(&mut item);
      }

      Some(item)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct FileItem {
  pub file_name: String,
  pub metadata: Metadata,
  pub path: PathBuf,
  pub is_dir: bool,
  pub level: usize,
  pub is_empty: bool,
  pub is_last: bool,
}

impl FileItem {
  fn new(p: PathBuf) -> FileItem {
    let metadata = p.symlink_metadata().unwrap();
    FileItem {
      file_name: p.file_name().unwrap().to_str().unwrap().to_string(),
      path: p.clone(),
      is_dir: metadata.is_dir(),
      level: 0,
      is_empty: true,
      is_last: false,
      metadata,
    }
  }
  fn set_is_empty(&mut self, b: bool) -> &mut FileItem {
    self.is_empty = b;
    self
  }
  fn set_is_last(&mut self, b: bool) -> &mut FileItem {
    self.is_last = b;
    self
  }
  fn set_level(&mut self, level: usize) -> &mut FileItem {
    self.level = level;
    self
  }
}
