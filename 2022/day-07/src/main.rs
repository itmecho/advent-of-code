use std::{cell::RefCell, collections::HashMap, rc::Rc};

const INPUT: &'static str = include_str!("../input.txt");

type NodeRef = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    children: HashMap<String, NodeRef>,
    parent: Option<NodeRef>,
    size: usize,
}

impl Node {
    pub fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    pub fn size_on_disk(&self) -> usize {
        self.size
            + self
                .children
                .iter()
                .map(|(_, node)| node.borrow().size_on_disk())
                .sum::<usize>()
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut current = root.clone();

    for line in INPUT.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        match tokens[0] {
            "$" => {
                if tokens[1] == "cd" {
                    match tokens[2] {
                        "/" => current = root.clone(),
                        ".." => {
                            let parent = current.borrow().parent.as_ref().unwrap().clone();
                            current = parent;
                        }
                        name => {
                            let target = current.borrow().children.get(name).unwrap().clone();
                            current = target;
                        }
                    }
                }
            }
            "dir" => {
                let node = Rc::new(RefCell::new(Node {
                    parent: Some(current.clone()),
                    ..Default::default()
                }));
                current
                    .borrow_mut()
                    .children
                    .insert(tokens[1].to_string(), node);
            }
            size => {
                let size = size.parse::<usize>().unwrap();
                let node = Rc::new(RefCell::new(Node {
                    parent: Some(current.clone()),
                    size,
                    ..Default::default()
                }));
                current
                    .borrow_mut()
                    .children
                    .insert(tokens[1].to_string(), node);
            }
        }
    }

    let mut dirs: Vec<usize> = Vec::new();
    process_dirs(root.clone(), &mut dirs);
    let answer_1: usize = dirs.iter().filter(|size| **size <= 100_000).sum();
    println!("part 1: {answer_1}");

    let used = root.borrow().size_on_disk();
    let free = 70_000_000 - used;
    let remaining = 30_000_000 - free;
    let answer_2 = dirs.iter().filter(|d| **d >= remaining).min().unwrap();
    println!("part 2: {answer_2}");
}

fn process_dirs(root: NodeRef, dirs: &mut Vec<usize>) {
    root.borrow().children.values().for_each(|node| {
        let node_borrow = node.borrow();
        if node_borrow.is_dir() {
            dirs.push(node_borrow.size_on_disk());
            process_dirs(node.clone(), dirs);
        }
    });
}
