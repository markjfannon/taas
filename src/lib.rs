use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::Serialize;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Tree {
    pub id: u32,
    pub age: TreeAge,
    pub trunk_width: u32,
    pub ward: String,
    pub species: String,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct TreeBinTree {
    data: Tree,
    left: Box<Option<TreeBinTree>>,
    right: Box<Option<TreeBinTree>>,
}

impl TreeBinTree {
    pub fn new(data: Tree) -> TreeBinTree {
        TreeBinTree {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn insert(&mut self, value: Tree) {
        if value.height > self.data.height {
            match self.right.as_mut() {
                Some(t) => t.insert(value),
                None => self.right = Box::new(Some(TreeBinTree::new(value))),
            }
        } else {
            match self.left.as_mut() {
                Some(t) => t.insert(value),
                None => self.left = Box::new(Some(TreeBinTree::new(value))),
            }
        }
    }

    fn get_min(&self) -> Tree {
        match self.left.as_ref() {
            Some(t) => t.get_min(),
            None => self.data.clone(),
        }
    }

    fn get_max(&self) -> Tree {
        match self.right.as_ref() {
            Some(t) => t.get_max(),
            None => self.data.clone(),
        }
    }

    fn get_size(&self) -> usize {
        let l_size = match self.left.as_ref() {
            Some(t) => t.get_size(),
            None => 0,
        };

        let r_size = match self.right.as_ref() {
            Some(t) => t.get_size(),
            None => 0,
        };

        1 + r_size + l_size
    }
}

#[derive(Debug, Clone)]
pub struct TreeTree {
    pub mature_trees: TreeBinTree,
    pub em_trees: TreeBinTree,
    pub sm_trees: TreeBinTree,
    pub young_trees: TreeBinTree,
}

impl TreeTree {
    pub fn get_min(&self, age: TreeAge) -> Tree {
        match age {
            TreeAge::Mature => self.mature_trees.get_min(),
            TreeAge::EarlyMature => self.em_trees.get_min(),
            TreeAge::SemiMature => self.sm_trees.get_min(),
            TreeAge::Young => self.young_trees.get_min(),
        }
    }

    pub fn get_max(&self, age: TreeAge) -> Tree {
        match age {
            TreeAge::Mature => self.mature_trees.get_max(),
            TreeAge::EarlyMature => self.em_trees.get_max(),
            TreeAge::SemiMature => self.sm_trees.get_max(),
            TreeAge::Young => self.young_trees.get_max(),
        }
    }

    pub fn get_size(&self, age: TreeAge) -> usize {
        match age {
            TreeAge::Mature => self.mature_trees.get_size(),
            TreeAge::EarlyMature => self.em_trees.get_size(),
            TreeAge::SemiMature => self.sm_trees.get_size(),
            TreeAge::Young => self.young_trees.get_size(),
        }
    }
}

#[derive(Debug, serde::Deserialize, Serialize, Clone, Copy)]
pub enum TreeAge {
    Mature,
    EarlyMature,
    SemiMature,
    Young,
}

// Deserialises room field of Excel file into a vector of structs
pub fn deserialise_trees(lines: i32) -> Vec<Tree> {
    let mut wb: Xlsx<_> = open_workbook("trees.xlsx").expect("Cannot open file");
    let range = wb.worksheet_range("trees").unwrap();
    let mut trees: Vec<Tree> = Vec::new();

    let iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();

    for (counter, result) in iter.enumerate() {
        if counter as i32 == lines {
            break;
        }

        if let Ok(tree) = result {
            trees.push(tree)
        }
    }

    trees
}

pub fn prepare_tree(trees: Vec<Tree>) -> TreeTree {
    let mut m_trees: Vec<Tree> = Vec::new();
    let mut em_trees: Vec<Tree> = Vec::new();
    let mut sm_trees: Vec<Tree> = Vec::new();
    let mut y_trees: Vec<Tree> = Vec::new();

    for tree in trees {
        if !(tree.height == 0 || tree.trunk_width == 0) {
            match tree.age {
                TreeAge::Mature => m_trees.push(tree),
                TreeAge::EarlyMature => em_trees.push(tree),
                TreeAge::SemiMature => sm_trees.push(tree),
                TreeAge::Young => y_trees.push(tree),
            }
        }
    }

    m_trees.sort_by_key(|tree| tree.height);
    em_trees.sort_by_key(|tree| tree.height);
    sm_trees.sort_by_key(|tree| tree.height);
    y_trees.sort_by_key(|tree| tree.height);

    let m_root = m_trees.remove(m_trees.len() / 2);
    let em_root = em_trees.remove(em_trees.len() / 2);
    let sm_root = sm_trees.remove(sm_trees.len() / 2);
    let y_root = y_trees.remove(y_trees.len() / 2);

    let mut tt = TreeTree {
        mature_trees: TreeBinTree::new(m_root),
        em_trees: TreeBinTree::new(em_root),
        sm_trees: TreeBinTree::new(sm_root),
        young_trees: TreeBinTree::new(y_root),
    };

    while let Some(element) = m_trees.pop() {
        tt.mature_trees.insert(element);
    }

    while let Some(element) = em_trees.pop() {
        tt.em_trees.insert(element);
    }

    while let Some(element) = sm_trees.pop() {
        tt.sm_trees.insert(element);
    }

    while let Some(element) = y_trees.pop() {
        tt.young_trees.insert(element);
    }

    tt
}
