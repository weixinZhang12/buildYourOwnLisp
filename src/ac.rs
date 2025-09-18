use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};
type NodeRef = Rc<RefCell<AcNode>>;
type NodeWeakRef = Weak<RefCell<AcNode>>;
// 8+8+8+8=32byte
///ac自动机节点
#[derive(Debug)]
pub struct AcNode {
    // 匹配失败的时候跳转位置,fail指针一定存在,这里有option是因为初始化的时候rust必须设置默认值
    fail: Option<Weak<RefCell<AcNode>>>,
    // 上一个节点
    last: Option<Weak<RefCell<AcNode>>>,
    // 子节点
    child: HashMap<char, Rc<RefCell<AcNode>>>,
    is_word: bool,
}

impl AcNode {
    ///创建一个ac自动机节点
    pub fn new() -> NodeRef {
        let sself = Self {
            last: None,
            fail: None,
            is_word: false,
            child: HashMap::new(),
        };
        Rc::new(RefCell::new(sself))
    }
    // 获取当前节点的前一个节点
    pub fn get_prev_node(node: &NodeRef) -> Option<NodeRef> {
        let node = node.borrow().last.clone()?;
        node.upgrade()
    }
    ///获取传入节点的fail指向的节点
    pub fn get_fail_node(node: &NodeRef) -> Option<NodeRef> {
        let node = node.borrow().fail.clone()?;
        node.upgrade()
    }
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
            AcNode::push_str(node, i);
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
                    let new_node = AcNode::new();
                    // 如果是最后一个元素那么就添加是个单词

                    current_borrow.child.insert(c, new_node.clone());
                    new_node
                }
            };
            if len - 1 == index {
                next_node.borrow_mut().is_word = true;
            }
            // 将下一个节点的父节点指针指向当前节点
            next_node.borrow_mut().last = Some(Rc::downgrade(&current_node));
            current_node = next_node;
        }
    }
    fn set_fail(node: &NodeRef) {
        // 将根节点fail;指针设置为自身
        node.borrow_mut().fail = Some(Rc::downgrade(node));
        Self::__set_fail(node, node);
    }

    fn __set_fail(node: &NodeRef, head: &NodeRef) {
        // let node_ref=node.borrow();
        let mut deque = VecDeque::new();
        let mut char_deque = VecDeque::new();
        // 将头节点入队
        deque.push_back(node.clone());
        // 头节点出对
        while let Some(front_node) = deque.pop_front() {
            {
                let node_ref = front_node.borrow();
                // 将头节点的所有子节点进入队列
                for (key, value) in node_ref.child.iter() {
                    deque.push_back(value.clone());
                    char_deque.push_back(key.to_owned());
                }
            }
            // 获取前一个节点，如果没有前一个节点的一定是头节点
            let prev_node = AcNode::get_prev_node(&front_node);
            // 不是头节点的情况
            if let Some(prev_node) = prev_node {
            let c = char_deque.pop_front().expect("不存在char");
                // 树中不可能存在不存在fail节点的节点
                let fail_node = AcNode::get_fail_node(&prev_node).expect("不存在fail指针");
                // 查看fail指针指向的节点是否有相同但不是自身的节点
                match fail_node.borrow().child.get(&c) {
                    Some(find_node) => {
                        // 如果查找到的节点和当前节点是同一个节点，那么fail指向根节点
                        if !Rc::ptr_eq(find_node, &front_node) {
                            // 安全代码
                            front_node.borrow_mut().fail = Some(Rc::downgrade(find_node))
                        } else {
                            front_node.borrow_mut().fail = Some(Rc::downgrade(head))
                        }
                    }
                    None => {
                        // 如果fail子节点没有相同的就指向根节点
                        // 安全代码
                        // node_ref.fail = Some(Rc::downgrade(node))
                        front_node.borrow_mut().fail = Some(Rc::downgrade(head));
                    }
                }
            }
            // 是头节的情况
            else {
                front_node.borrow_mut().fail = Some(Rc::downgrade(head));
            }
        }
        // 获取头节点
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
    let root = AcNode::new();
    AcNode::push_str(&root, "her");
    AcNode::push_str(&root, "his");
    AcNode::push_str(&root, "he");
    AcNode::push_str(&root, "hi");
    AcNode::push_str(&root, "what");
    AcNode::push_str(&root, "next");
    AcNode::push_str_by_arr(&root, vec!["apple", "next", "ppater"]);
    AcNode::set_fail(&root);
    assert!(!AcNode::pattern(&root, "Heo"));
    assert!(AcNode::pattern(&root, "his"));
    assert!(AcNode::pattern(&root, "he"));
    assert!(!AcNode::pattern(&root, "hell"));
    assert!(!AcNode::pattern(&root, "nex"));
    println!("{:#?}", root.borrow());
    
}

#[test]
fn struct_sample_test() {
    let root = AcNode::new();
    AcNode::push_str(&root, "her");
    AcNode::set_fail(&root);
    println!("{:#?}", root.borrow());
    
}

