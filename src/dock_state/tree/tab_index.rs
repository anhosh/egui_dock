use crate::{NodeIndex, NodePath, SurfaceIndex};

/// Identifies a tab within a [`Node`](crate::Node).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabIndex(pub usize);

impl From<usize> for TabIndex {
    #[inline]
    fn from(index: usize) -> Self {
        TabIndex(index)
    }
}

/// A full path to locate a tab within an entire dock state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TabPath {
    /// Index of the surface owning the node for the tab.
    pub surface: SurfaceIndex,
    /// Index of the node for the tab in the surface tree.
    pub node: NodeIndex,
    /// Index of the tab in the node.
    pub tab: TabIndex,
}

impl TabPath {
    /// Creates a new fully qualified path to a tab.
    pub const fn new(surface: SurfaceIndex, node: NodeIndex, tab: TabIndex) -> Self {
        Self { surface, node, tab }
    }

    /// Get the node path components.
    pub fn node_path(self) -> NodePath {
        NodePath {
            surface: self.surface,
            node: self.node,
        }
    }
}

impl From<(NodePath, TabIndex)> for TabPath {
    fn from((NodePath { surface, node }, tab): (NodePath, TabIndex)) -> Self {
        TabPath { surface, node, tab }
    }
}
