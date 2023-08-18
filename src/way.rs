use crate::node_ref_list::WayNodeList;

/// A way is a tagged list of [Nodes](crate::Node).
pub enum Way {}

impl Way {
    /// Get the way's nodes
    pub fn nodes(&self) -> &WayNodeList {
        unsafe { way_nodes_const(self) }
    }

    /// Get the mutable way's nodes
    pub fn nodes_mut(&mut self) -> &mut WayNodeList {
        unsafe { way_nodes(self) }
    }
}

extern "C" {
    fn way_nodes_const(way: &Way) -> &WayNodeList;
    fn way_nodes(way: &mut Way) -> &mut WayNodeList;
}
