use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
struct Directory<'a> {
    pub files: Vec<(u32, &'a str)>,
    pub directories: HashMap<&'a str, Rc<RefCell<Directory<'a>>>, RandomState>,
    pub parent: Option<Weak<RefCell<Directory<'a>>>>,
}

impl<'a> Directory<'a> {
    fn calculate_total_size(&self) -> u32 {
        self.files.iter().map(|(size, _)| size).sum::<u32>()
            + self
                .directories
                .iter()
                .map(|(_, directory)| directory.borrow().calculate_total_size())
                .sum::<u32>()
    }

    fn calculate_total_size_sum(&self) -> u32 {
        self.directories
            .iter()
            .map(|(_, directory)| {
                let mut size = directory.borrow().calculate_total_size();
                if size > 100000 {
                    size = 0;
                }
                size += directory.borrow().calculate_total_size_sum();
                size
            })
            .sum::<u32>()
    }
}

fn compute(content: String) -> String {
    let root_directory = Rc::new(RefCell::new(Directory {
        files: vec![],
        directories: HashMap::new(),
        parent: None,
    }));
    let mut current_directory = Rc::clone(&root_directory);
    content.split("$ ").skip(2).for_each(|command| {
        let (command, results) = command.split_once('\n').unwrap();
        match &command[..2] {
            "ls" => {
                let mut files = vec![];
                let mut directories = HashMap::new();
                results.lines().for_each(|result| {
                    let (first, second) = result.split_once(' ').unwrap();
                    match first {
                        "dir" => {
                            directories.insert(
                                second,
                                Rc::new(RefCell::new(Directory {
                                    files: vec![],
                                    directories: HashMap::new(),
                                    parent: Some(Rc::downgrade(&current_directory)),
                                })),
                            );
                        }
                        _ => {
                            files.push((first.parse::<u32>().unwrap(), second));
                        }
                    }
                });
                current_directory.borrow_mut().directories = directories;
                current_directory.borrow_mut().files = files;
            }
            "cd" => {
                let (_, directory) = command.split_at(3);
                match directory {
                    ".." => {
                        current_directory = current_directory
                            .to_owned()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap();
                    }
                    directory => {
                        current_directory = Rc::clone(
                            current_directory
                                .to_owned()
                                .borrow()
                                .directories
                                .get(directory)
                                .unwrap(),
                        );
                    }
                }
            }
            _ => panic!("Unknown command"),
        }
    });
    root_directory
        .to_owned()
        .borrow()
        .calculate_total_size_sum()
        .to_string()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let result = compute(content);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let content = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        let expected = "95437";
        assert_eq!(compute(content.to_string()), expected);
    }
}
