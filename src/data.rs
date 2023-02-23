use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_end_of_word: bool,
}
impl Node {
    fn new() -> Node {
        Node {
            children: HashMap::new(), // 26 letters in English alphabet
            is_end_of_word: false,
        }
    }
}
pub struct Trie {
    root: Node,
}
impl Trie {
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert_with(|| Node::new());
        }
        current_node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            if let Some(child) = current_node.children.get(&c) {
                current_node = child;
            } else {
                return false;
            }
        }
        current_node.is_end_of_word
    }
}
