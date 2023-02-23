struct TreeNode {
    c: char,
    children: Vec<char>,
}

impl TreeNode {
    fn new(c: char) -> Self {
        TreeNode {
            c,
            children: Vec::new()
        }
    }

    pub fn build_tree(text: &str) -> Vec<Self> {
        let chars = text.chars();
        let mut nodes: Vec<TreeNode> = Vec::new();
        let mut cur_node: Option<&mut TreeNode> = None;
        
        while let c = chars.next() {
            if cur_node == None {
                for node in &mut nodes {
                    if node.c == c {
                        
                    }
                }
            }
        }
    }
}
