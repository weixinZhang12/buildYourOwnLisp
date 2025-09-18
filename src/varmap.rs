use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};
type NodeRef = Rc<RefCell<VarNode>>;
type NodeWeakRef = Weak<RefCell<VarNode>>;
// 8+8+8+8=32byte
///ac自动机节点
#[derive(Debug)]
pub struct VarNode {
    // 子节点
    child: HashMap<char, Rc<RefCell<VarNode>>>,
    is_word: bool,
}

impl VarNode {
    ///创建一个ac自动机节点
    pub fn new() -> NodeRef {
        let sself = Self {
            is_word: false,
            child: HashMap::new(),
        };
        Rc::new(RefCell::new(sself))
    }
    // 获取当前节点的前一个节点

    // 匹配字符串
    pub fn pattern<S: AsRef<str>>(node: &NodeRef, s: S) -> bool {
        let s = s.as_ref();
        let mut current_node = node.clone();
        for c in s.chars() {
            // 查看有没有当前的单词节点
            let next = {
                match current_node.borrow().child.get(&c) {
                    Some(v) => v.clone(),
                    None => return false,
                }
            };
            current_node = next;
        }
        // 这里的当前节点就是查询到的最后一个节点
        current_node.borrow().is_word
    }
    pub fn push_str_by_arr<S: AsRef<str>>(node: &NodeRef, vec: Vec<S>) {
        for i in vec {
            VarNode::push_str(node, i);
        }
    }
    ///插入查找词
    pub fn push_str<S: AsRef<str>>(node: &NodeRef, s: S) {
        let s = s.as_ref();
        let len = s.len();
        if len == 0 {
            return;
        }
        // 当前节点等于传入的节点
        let mut current_node = node.clone();
        let iter = s.chars();
        for (index, c) in s.chars().enumerate() {
            // 获取下一个节点
            let next_node = {
                let mut current_borrow = current_node.borrow_mut();
                // 如果子节点有当前需要插入的字符，那么下一个节点就是查找到的子节点。否则插入一个新节点，将新节点设置为下一个节点并返回
                if let Some(existing_node) = current_borrow.child.get(&c) {
                    existing_node.clone()
                } else {
                    let new_node = VarNode::new();
                    // 如果是最后一个元素那么就添加是个单词

                    current_borrow.child.insert(c, new_node.clone());
                    new_node
                }
            };
            if len - 1 == index {
                next_node.borrow_mut().is_word = true;
            }
            // 将下一个节点的父节点指针指向当前节点
            current_node = next_node;
        }
    }
}
pub struct AcManger {
    current_node: NodeRef,
    head: NodeRef,
}
impl AcManger {}

// 使用示例
#[test]
fn test() {
    let root = VarNode::new();
    VarNode::push_str(&root, "her");
    VarNode::push_str(&root, "his");
    VarNode::push_str(&root, "he");
    VarNode::push_str(&root, "hi");
    VarNode::push_str(&root, "what");
    VarNode::push_str(&root, "next");
    VarNode::push_str_by_arr(&root, vec!["apple", "next", "ppater"]);
    assert!(!VarNode::pattern(&root, "Heo"));
    assert!(VarNode::pattern(&root, "his"));
    assert!(VarNode::pattern(&root, "he"));
    assert!(!VarNode::pattern(&root, "hell"));
    assert!(!VarNode::pattern(&root, "nex"));
    println!("{:#?}", root.borrow());
    
}

#[test]
fn struct_sample_test() {
    let root = VarNode::new();
    VarNode::push_str(&root, "her");
    println!("{:#?}", root.borrow());
    
}

