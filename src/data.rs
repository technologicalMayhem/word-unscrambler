struct Node { children: Vec<Option<Box<Node>>>, 
    is_end_of_word: bool,
}
impl Node { fn new() -> Node { Node { children: vec![None; 
            26], // 26 letters in English alphabet 
            is_end_of_word: false,
        }
    }
}
struct Trie { root: Node,
}
impl Trie { fn new() -> Trie { Trie { root: Node::new() }
    }
    
    fn insert(&mut self, word: &str) { let mut current_node 
        = &mut self.root; for c in word.chars() {
            let index = (c as u8 - b'a') as usize; // 
            convert char to index if 
            current_node.children[index].is_none() {
                current_node.children[index] = 
                Some(Box::new(Node::new()));
            }
            current_node = 
            current_node.children[index].as_mut().unwrap();
        }
        current_node.is_end_of_word = true;
    }
    
    fn search(&self, word: &str) -> bool { let mut 
        current_node = &self.root; for c in word.chars() {
            let index = (c as u8 - b'a') as usize; // 
            convert char to index if 
            current_node.children[index].is_none() {
                return false;
            }
            current_node = 
            current_node.children[index].as_ref().unwrap();
        }
        current_node.is_end_of_word
    }
}
