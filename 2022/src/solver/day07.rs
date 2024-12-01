pub enum Command<'a> {
    Ls,
    Cd(&'a str),
}

fn parse_command<'a>(cmd: &'a str) -> Command<'a> {
    if cmd.starts_with("cd") {
        return Command::Cd(&cmd[3..])
    } else {
        return Command::Ls
    }
}

pub enum Entry<'a> {
    Dir,
    File(u64, &'a str),
}

fn parse_entry<'a>(entry: &'a str) -> Entry<'a> {
    if entry.starts_with("dir") {
        return Entry::Dir
    } else {
        let (size, name) = entry.split_once(" ").unwrap();
        return Entry::File(size.parse::<u64>().unwrap(), name)
    }
}

pub enum List<'a> {
    Command(Command<'a>),
    Entry(Entry<'a>),
}

pub fn parse(input: &str) -> Vec<List> {
    input.lines().map(|s| -> List {
        if s.starts_with("$") {
            return List::Command(parse_command(&s[2..]))
        }

        List::Entry(parse_entry(s))
    }).collect::<Vec<_>>()
}

struct Node<'a> {
    _path: &'a str,
    size: u64,
    children: Vec<Node<'a>>,
}

impl<'a> Default for Node<'a> {
    fn default() -> Self {
        Self { _path: "/".into(), size: 0, children: Vec::new() }
    }
}

impl<'a> Node<'a> {
    fn is_dir(&self) -> bool {
        !self.children.is_empty()
    }

    fn traverse_node_sizes(&self, sizes: &mut Vec<u64>) -> u64 {
        let current_size = self.size + self.children
            .iter()
            .map(|node| {
                if node.is_dir() {
                    return node.traverse_node_sizes(sizes)
                }

                node.size
            })
            .sum::<u64>();
        
        sizes.push(current_size);

        current_size
    }
}

pub fn solve(input: &Vec<List>) -> (u64, u64) {
    let mut stack_node = vec![Node::default()];

    for line in input {
        match line {
            List::Command(cmd) => match cmd {
                Command::Ls => {},
                Command::Cd(path) => match *path {
                    "/" => {},
                    ".." => {
                        let last = stack_node.pop().unwrap();
                        stack_node.last_mut().unwrap().children.push(last)
                    },
                    _ => {
                        let node = Node{
                            size: 0,
                            _path: path,
                            children: vec![],
                        };

                        stack_node.push(node);
                    },
                },
            },
            List::Entry(entry) => match entry {
                Entry::Dir => {},
                Entry::File(size, path) => {
                    let node = Node{
                        size: *size,
                        _path: path,
                        children: vec![],
                    };

                    stack_node.last_mut().unwrap().children.push(node)
                },
            },
        }
    }

    let mut root = stack_node.pop().unwrap();
    while let Some(mut node) = stack_node.pop() {
        node.children.push(root);
        root = node;
    }

    let mut sizes = Vec::<u64>::new();
    _ = root.traverse_node_sizes(&mut sizes);


    let part1 = sizes.iter().filter(|&&s| s <= 100_000).sum::<u64>();

    let needed = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    let part2 = *sizes.iter().filter(|&&s| s >= needed).min().unwrap();

    (part1, part2)
}