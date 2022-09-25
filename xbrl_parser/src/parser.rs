use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
use roxmltree::{Document, ExpandedName};

use crate::node::XBRLNode;

pub fn read_file<P>(path: P) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    Ok(buff)
}

pub fn parse_xml(text: &str) -> anyhow::Result<Document> {
    let xml = Document::parse(text)?;
    Ok(xml)
}

///　リンクファイル(pre.xml)読み込み
pub fn read_linkfile(text: &str) -> anyhow::Result<XBRLNode> {
    let xml = parse_xml(text)?;

    // 名前空間名取得
    let xlink = xml
        .root_element()
        .namespaces()
        .iter()
        .find(|namespace| namespace.name() == Some("xlink"))
        .ok_or_else(|| anyhow!("namespac xlink not found"))?
        .uri();

    // BalanceSheetのリンク一覧を取得
    let pre_def: Vec<roxmltree::Node> = xml
        .root_element()
        .children()
        .find(|element| {
            element.tag_name().name() == "presentationLink"
                && element.attribute(ExpandedName::from((xlink, "role")))
                    == Some("http://disclosure.edinet-fsa.go.jp/role/jppfs/rol_BalanceSheet")
        })
        .ok_or_else(|| anyhow!("BalanceSheet presentationLink not found"))?
        .children()
        .collect();

    let mut root = XBRLNode::default();

    for (i, node) in pre_def
        .iter()
        .filter(|element| element.tag_name().name() == "presentationArc")
        .enumerate()
    {
        // 親要素
        let parent = pre_def
            .iter()
            .find(|element| {
                element.tag_name().name() == "loc"
                    && element
                        .attribute(ExpandedName::from((xlink, "label")))
                        .expect("attribute name label not found")
                        == node
                            .attribute(ExpandedName::from((xlink, "from")))
                            .expect("attribute name from not found")
            })
            .ok_or_else(|| anyhow!("element name loc not found"))?;

        // 子要素
        let children = pre_def
            .iter()
            .clone()
            .find(|element| {
                element.tag_name().name() == "loc"
                    && element
                        .attribute(ExpandedName::from((xlink, "label")))
                        .expect("attribute name label not found")
                        == node
                            .attribute(ExpandedName::from((xlink, "to")))
                            .expect("attribute name to not found")
            })
            .ok_or_else(|| anyhow!("element name loc not found"))?;
        let children_order = node
            .attribute("order")
            .ok_or_else(|| anyhow!("attribute order not found"))?;

        let parent = XBRLNode::new(i.to_string(), parent.clone(), xlink);
        let children = XBRLNode::new(children_order.to_string(), children.clone(), xlink);

        // 親要素,子要素を追加
        root.insert(parent, children)?;
    }

    Ok(root)
}
