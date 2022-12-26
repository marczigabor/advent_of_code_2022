use std::{fs::File, rc::Rc, cell::RefCell};
#[path = "../../src/utils/mod.rs"] mod utils;

#[derive(Debug)]
enum ObjType {
    File,
    Folder,
}

#[derive(Debug)]
struct Value {
    pub name: Option<String>,
    pub size: Option<u32>,
    pub obj_type: Option<ObjType>
}

impl Value {
    pub fn new() -> Value {
      return Value {
        name: None,
        size: None,
        obj_type: None,
      };
    }
}


#[derive(Debug)]
struct TreeNode {
    pub value:  Option<Rc<RefCell<Value>>>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
      return TreeNode {
        value: None,
        children: vec![],
        parent: None,
      };
    }
  
    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
      self.children.push(new_node);
    }
}

pub fn day_07_01() -> usize {
    //let file_path = "src/day_07/data.txt";
    //let contents;
    //contents = utils::read_file(file_path);

    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    
    let child = Rc::new(RefCell::new(TreeNode::new()));
    current.borrow_mut().children.push(Rc::clone(&child));

    {
        let mut mut_child = child.borrow_mut();
        mut_child.parent = Some(Rc::clone(&current));

        let v = Value {
            name: Some(String::from("test")), 
            obj_type: Some(ObjType::File), 
            size: Some(5000) 
        };
        
        mut_child.value =  Some(Rc::new(RefCell::new(v)));

        //mut_child.value = Some(Rc::clone(&current));

    }
    current = child;

    // for line in contents.lines().into_iter() {

    // }
    0
}

pub fn day_07_02() -> usize {
    // let file_path = "src/day_06/data.txt";
    // let contents;
    // contents = utils::read_file(file_path);
   0

}