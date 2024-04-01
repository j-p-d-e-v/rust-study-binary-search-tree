use std::mem;
use std::cell::RefCell;

type Tree = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree
}

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

#[derive(Clone,Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

impl Node {
    fn new(dev: IoTDevice) -> Tree {
        let node: Node = Node {
            dev: dev,
            left: None,
            right: None,
        };
        Some(Box::new(node))
    }
}

impl BinarySearchTree {

    fn new() -> BinarySearchTree {
        BinarySearchTree { root: None, length: 0 }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            },
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }
    
    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                }
                else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                }
                else{
                    self.find_r(&n.right, numerical_id)
                }
            },
            _ => None,
        }
    }

    pub fn walk<F>(&self,callback: &mut F)
    where
        F: FnMut(&IoTDevice) -> ()
    {
        self.walk_in_order(&self.root, callback);
    }

    fn walk_in_order<F>(&self, node: &Tree,callback: &mut F)
    where
        F: FnMut(&IoTDevice) -> ()
    {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
    
}

fn main() {
    let root_device: IoTDevice = IoTDevice {
        numerical_id: 10,
        address: String::from("1.1.1.10")
    };
    let device_1: IoTDevice = IoTDevice {
        numerical_id: 5,
        address: String::from("1.1.1.5")
    };
    let device_2: IoTDevice = IoTDevice {
        numerical_id: 8,
        address: String::from("1.1.1.8")
    };
    let device_3: IoTDevice = IoTDevice {
        numerical_id: 11,
        address: String::from("1.1.1.11")
    };
    let mut bs_tree = BinarySearchTree::new();    
    bs_tree.add(root_device);
    bs_tree.add(device_1.clone());
    bs_tree.add(device_2);
    bs_tree.add(device_3);
    println!("Tree: {:#?}",bs_tree);
    println!("Find: {:#?}",bs_tree.find(5));

    let mut iot_devices:  Vec<IoTDevice> = Vec::new();
    bs_tree.walk(&mut |n: &IoTDevice| {
        iot_devices.push(n.clone());
    });
    println!("Walk: {:#?}",iot_devices);
}
