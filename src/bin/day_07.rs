use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod input {
    pub struct Invocation {
        pub command: Command,
        pub output: Vec<FsEntry>,
    }

    impl Invocation {
        fn new(command: Command, output: Vec<FsEntry>) -> Self {
            Self { command, output }
        }
    }

    pub enum Command {
        Ls,
        CdRoot,
        CdParent,
        CdDir(String),
    }

    impl Command {
        fn new_cd(arg: &str) -> Self {
            match arg {
                "/" => Self::CdRoot,
                ".." => Self::CdParent,
                _ => Self::CdDir(arg.to_string()),
            }
        }
    }

    pub enum FsEntry {
        Dir(String),
        File(String, usize),
    }

    pub fn parse_input() -> Vec<Invocation> {
        let input = include_str!("../../data/day_07.txt");

        peg::parser! {
            grammar parser() for str {
                pub(crate) rule input() -> Vec<Invocation>
                    = i:invocation() ++ "\r\n" ![_] { i }

                rule invocation() -> Invocation
                    = c:cd() { c }
                    / l:ls() { l }

                rule cd() -> Invocation
                    = "$ cd " arg:$([^ '\r' | '\n']+) {
                        Invocation::new(Command::new_cd(arg), Default::default())
                    }

                rule ls() -> Invocation
                    = "$ ls\r\n" e:entries() {
                        Invocation::new(Command::Ls, e)
                    }

                rule entries() -> Vec<FsEntry>
                    = e:entry() ++ "\r\n" { e }

                rule entry() -> FsEntry
                    = size:$(['0'..='9']+) " " name:$([^ '\r' | '\n']+) { FsEntry::File(name.to_string(), size.parse().unwrap()) }
                    / "dir " name:$([^ '\r' | '\n']+) { FsEntry::Dir(name.to_string()) }
            }
        }

        parser::input(input).unwrap()
    }
}

struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, usize>,
    size: Option<usize>,
}

impl Dir {
    fn new(parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Self {
            parent,
            dirs: Default::default(),
            files: Default::default(),
            size: None,
        }
    }

    fn size(&mut self) -> usize {
        if let Some(size) = self.size {
            return size;
        }
        let size = self.calculate_size();
        self.size = Some(size);
        size
    }

    fn calculate_size(&self) -> usize {
        let mut result = 0usize;
        for dir in self.dirs.values() {
            result += dir.borrow_mut().size()
        }
        for file_size in self.files.values() {
            result += file_size
        }
        result
    }
}

struct Filesystem {
    root: Rc<RefCell<Dir>>,
    current_dir: Rc<RefCell<Dir>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
}

impl Filesystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::new(None)));
        Self {
            root: root.clone(),
            current_dir: root,
            dirs: Default::default(),
        }
    }

    fn incorporate(&mut self, invocation: &input::Invocation) {
        match &invocation.command {
            input::Command::CdRoot => self.current_dir = self.root.clone(),
            input::Command::CdParent => {
                let new_dir = self.current_dir.borrow().parent.as_ref().unwrap().clone();
                self.current_dir = new_dir;
            }
            input::Command::CdDir(name) => {
                let new_dir = self.current_dir.borrow().dirs.get(name).unwrap().clone();
                self.current_dir = new_dir;
            }
            input::Command::Ls => {
                for entry in invocation.output.iter() {
                    self.add_entry(&mut self.current_dir.clone(), entry);
                }
            }
        }
    }

    fn add_entry(&mut self, target: &mut Rc<RefCell<Dir>>, entry: &input::FsEntry) {
        match entry {
            input::FsEntry::Dir(name) => {
                let new_dir = Rc::new(RefCell::new(Dir::new(Some(target.clone()))));
                target
                    .borrow_mut()
                    .dirs
                    .entry(name.clone())
                    .or_insert_with(|| new_dir.clone());
                self.dirs.push(new_dir);
            }
            input::FsEntry::File(name, size) => {
                target
                    .borrow_mut()
                    .files
                    .entry(name.clone())
                    .or_insert(*size);
            }
        }
    }
}

fn part1() -> usize {
    let mut filesystem = Filesystem::new();

    for invocation in input::parse_input().iter() {
        filesystem.incorporate(invocation);
    }

    filesystem
        .dirs
        .iter()
        .map(|dir| dir.borrow_mut().size())
        .filter(|size| *size < 100000)
        .sum()
}

fn part2() -> usize {
    let mut filesystem = Filesystem::new();

    for invocation in input::parse_input().iter() {
        filesystem.incorporate(invocation);
    }

    // Need 30000000 free and filesystem has total capacity of 70000000.  Therefore any
    // usage beyond (70000000 - 30000000) = 40000000 has to be deleted.
    let must_delete_at_least = filesystem.root.borrow_mut().size() - 40000000;

    filesystem
        .dirs
        .iter()
        .map(|dir| dir.borrow_mut().size())
        .filter(|size| *size >= must_delete_at_least)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1501149);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 10096985);
    }
}
