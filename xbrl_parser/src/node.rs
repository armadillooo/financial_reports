use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use anyhow::anyhow;
use roxmltree::{ExpandedName, Node};

// xbrlインスタンス文書の要素
#[derive(Debug, Clone, Default)]
pub struct XBRLNode {
    pub name: String,
    pub label: String,
    pub location: String,
    pub order: String,
    /// 親要素
    parent: Option<Weak<RefCell<XBRLNode>>>,
    /// 子要素
    childrens: Vec<Rc<RefCell<XBRLNode>>>,
}

impl XBRLNode {
    /// コンストラクタ
    pub fn new(order: String, node: Node, xlink: &str) -> Self {
        let name = node
            .attribute(ExpandedName::from((xlink, "href")))
            .expect("attribute href not found")
            .split("#")
            .last()
            .unwrap()
            .to_owned();

        let label = node
            .attribute(ExpandedName::from((xlink, "label")))
            .unwrap()
            .to_owned();

        let location = node
            .attribute(ExpandedName::from((xlink, "href")))
            .unwrap()
            .to_owned();

        Self {
            name,
            label,
            location,
            parent: None,
            childrens: Vec::new(),
            order,
        }
    }

    /// ノードのルートからの深さを取得
    pub fn depth(&self) -> i32 {
        if let Some(node) = &self.parent {
            if let Some(node) = node.upgrade() {
                node.borrow_mut().depth() + 1
            } else {
                0
            }
        } else {
            0
        }
    }

    /// 要素検索
    pub fn find(&self, target: &XBRLNode) -> Option<Rc<RefCell<XBRLNode>>> {
        for child in self.childrens.iter() {
            if child.borrow().name == target.name {
                return Some(Rc::clone(child));
            } else {
                if let Some(decendant) = child.borrow().find(&target) {
                    return Some(decendant);
                }
            }
        }

        None
    }

    /// 要素追加
    pub fn insert(&mut self, from: XBRLNode, mut to: XBRLNode) -> anyhow::Result<()> {
        // 子要素が既に存在する
        if let Some(node) = self.find(&to) {
            return Err(anyhow!("Children node has already exists({:?})", node));
        }

        // 親要素が既に存在する
        let parent = if let Some(node) = self.find(&from) {
            node
        } else {
            self.childrens.push(Rc::new(RefCell::new(from)));
            Rc::clone(self.childrens.last().unwrap())
        };

        to.parent = Some(Rc::downgrade(&parent));
        parent
            .borrow_mut()
            .childrens
            .push(Rc::new(RefCell::new(to)));

        Ok(())
    }
}
