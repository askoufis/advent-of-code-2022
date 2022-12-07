use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Line {
    Ls,
    Cd(String),
    Dir(String),
    File(usize, String),
}

type NodeId = usize;

trait Node {
    fn get_size(&self, fs: &FileSystem) -> usize;
    fn get_name(&self) -> String;
}

struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: String::from(name),
            size,
        }
    }
}

impl Node for File {
    fn get_size(&self, _fs: &FileSystem) -> usize {
        self.size
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Clone)]
struct Directory {
    name: String,
    contents: HashSet<NodeId>,
    parent: NodeId,
}

impl Directory {
    fn new(name: &str, parent: NodeId) -> Self {
        Self {
            name: String::from(name),
            contents: HashSet::from([]),
            parent,
        }
    }

    fn add_item(mut self, node_id: NodeId) -> Self {
        self.contents.insert(node_id);
        self
    }
}

impl Node for Directory {
    fn get_size(&self, fs: &FileSystem) -> usize {
        self.contents
            .iter()
            .map(|node_id| {
                let item = fs.get_item(node_id);
                let size = item.get_size(fs);
                let name = item.get_name();
                println!("Size for '{}'({}): {}", name, node_id, size);
                size
            })
            .sum()
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

enum Item {
    File(File),
    Directory(Directory),
}

impl Item {
    fn new_file(name: &str, size: usize) -> Self {
        let file = File::new(name, size);
        Self::File(file)
    }

    fn new_directory(name: &str, parent: NodeId) -> Self {
        let directory = Directory::new(name, parent);
        Self::Directory(directory)
    }
}

impl Node for Item {
    fn get_size(&self, fs: &FileSystem) -> usize {
        match self {
            Item::File(file) => file.get_size(fs),
            Item::Directory(directory) => directory.get_size(fs),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Item::File(file) => file.get_name(),
            Item::Directory(directory) => directory.get_name(),
        }
    }
}

struct FileSystem {
    nodes: HashMap<NodeId, Item>,
    current_directory: NodeId,
    num_nodes: usize,
}

impl FileSystem {
    fn new() -> Self {
        let root_directory = Item::new_directory("/", 0);
        let mut nodes = HashMap::new();
        nodes.insert(0usize, root_directory);
        FileSystem {
            nodes,
            current_directory: 0,
            num_nodes: 1,
        }
    }

    fn get_item(&self, node_id: &NodeId) -> &Item {
        self.nodes.get(node_id).expect("Could not find node")
    }

    fn add_item(&mut self, item: Item) -> usize {
        let new_node_id = self.num_nodes;

        let current_directory_item = self.get_item(&self.current_directory);
        match current_directory_item {
            Item::File(_) => panic!("Expected dir, got file"),
            Item::Directory(dir) => {
                println!("Adding {} to dir {}", item.get_name(), dir.name);
                let mut new_dir = dir.clone().add_item(new_node_id);
                new_dir.contents.insert(new_node_id);
                self.nodes
                    .insert(self.current_directory, Item::Directory(new_dir));
            }
        }

        self.nodes.insert(new_node_id, item);
        self.num_nodes += 1;

        new_node_id
    }

    fn execute(&mut self, line: &Line) {
        match line {
            Line::Ls => {}
            Line::Cd(dir_name) => {
                if dir_name == ".." {
                    let current_directory_item = self.get_item(&self.current_directory);
                    match current_directory_item {
                        Item::File(_) => panic!("Expected dir, got file"),
                        Item::Directory(dir) => self.current_directory = dir.parent,
                    }
                } else {
                    let dir = Item::new_directory(&dir_name, self.current_directory);
                    self.current_directory = self.add_item(dir);
                }
            }
            Line::Dir(_) => {}
            Line::File(size, file_name) => {
                let file = Item::new_file(&file_name, *size);
                self.add_item(file);
            }
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let s = l.split(' ').into_iter();
            match s.clone().take(1).next() {
                Some(first) => {
                    if first == "$" {
                        let v: Vec<_> = s.skip(1).collect();

                        match v[0] {
                            "ls" => Line::Ls,
                            "cd" => Line::Cd(String::from(v[1])),
                            _ => panic!("Bad parse"),
                        }
                    } else {
                        let v: Vec<_> = s.collect();
                        match v[0] {
                            "dir" => Line::Dir(String::from(v[1])),
                            size => Line::File(size.parse().unwrap(), String::from(v[1])),
                        }
                    }
                }
                None => panic!("Bad parse"),
            }
        })
        // Skip first line as it's always "$ cd /", and we assume that we start in a root dir
        .skip(1)
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Line>) -> usize {
    let mut fs = FileSystem::new();

    input.iter().for_each(|line| fs.execute(line));

    fs.nodes
        .values()
        .filter_map(|item| {
            if let Item::Directory(directory) = item {
                let size = directory.get_size(&fs);
                println!("Directory '{}' has size {}", directory.get_name(), size);
                if size <= 100000 {
                    Some(size)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Line>) -> usize {
    let mut fs = FileSystem::new();

    input.iter().for_each(|line| fs.execute(line));

    let root_folder = fs.get_item(&0);
    let used_disk_space = root_folder.get_size(&fs);
    let current_unused_space = TOTAL_DISK_SPACE - used_disk_space;
    let space_to_free = GOAL_FREE_SPACE - current_unused_space;

    let dirs: Vec<Directory> = fs
        .nodes
        .iter()
        .filter_map(|(_, item)| match item {
            Item::File(_) => None,
            Item::Directory(dir) => Some(dir.clone()),
        })
        .collect();
    dirs.iter()
        .filter_map(|dir| {
            let size = dir.get_size(&fs);
            if size >= space_to_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

const TOTAL_DISK_SPACE: usize = 70000000;
const GOAL_FREE_SPACE: usize = 30000000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = input_generator(
            r"$ cd /
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
",
        );
        assert_eq!(part1(&input), 95437);
    }
}
