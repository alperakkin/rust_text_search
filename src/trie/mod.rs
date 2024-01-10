use std::collections::HashMap;

pub struct Node {
    children:  HashMap<char, Node>,
    end_word: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end_word: false,
        }
    }
}


pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(Node::new());
        }
        current_node.end_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for ch in word.chars(){
            if let Some(next_node) = current_node.children.get(&ch)
            {
                current_node = next_node;
            } else
            {
                return false;
            }
        }
        current_node.end_word
    }
}