use crate::SelectBox;
use osmpbf::{Element, WayRefIter};
use rijksdriehoek::wgs84_to_rijksdriehoek;
use std::collections::HashMap;

pub struct NodeIndex(HashMap<i64, (u32, u32)>);

impl NodeIndex {
    pub fn new() -> NodeIndex {
        NodeIndex(HashMap::new())
    }

    pub fn one(id: i64, x: u32, y: u32) -> NodeIndex {
        NodeIndex(HashMap::from([(id, (x, y))]))
    }

    pub fn combine(mut self, ni: NodeIndex) -> NodeIndex {
        self.0.extend(ni.0);
        self
    }

    pub fn filter(&self, select_box: &SelectBox) -> NodeIndex {
        let mut r = NodeIndex::new();

        for (&id, &(x, y)) in &self.0 {
            if select_box.is_inside(x, y) {
                r.0.insert(id, (x, y));
            }
        }

        r
    }

    /// svg path definition
    pub fn svg_path_d(&self, line: WayRefIter) -> String {
        let mut d = String::new();
        for (i, r) in line.enumerate() {
            let (x, y) = self.0.get(&r).copied().unwrap_or_default();
            if i == 0 {
                d += &format!("M {} -{} ", x, y);
            } else {
                d += &format!("L {} -{} ", x, y);
            }
        }

        d
    }

    pub fn convert(e: Element) -> NodeIndex {
        match e {
            Element::DenseNode(n) => {
                let (x, y) = wgs84_to_rijksdriehoek(n.lat(), n.lon());

                NodeIndex::one(n.id, x as u32, y as u32)
            }
            _ => NodeIndex::new(),
        }
    }

    pub fn contains_key(&self, key: i64) -> bool {
        self.0.contains_key(&key)
    }
}
