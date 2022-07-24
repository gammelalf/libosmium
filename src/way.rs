use crate::node_ref_list::WayNodeList;

pub enum Way {}

impl Way {
    pub fn nodes(&self) -> &WayNodeList {
        unsafe {
            way_nodes_const(self)
        }
    }

    pub fn nodes_mut(&mut self) -> &mut WayNodeList {
        unsafe {
            way_nodes(self)
        }
    }
}

extern "C" {
    fn way_nodes_const(way: &Way) -> &WayNodeList;
    fn way_nodes(way: &mut Way) -> &mut WayNodeList;
}