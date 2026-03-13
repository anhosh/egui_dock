use crate::{NodePath, SurfaceIndex, TabPath};

/// An enum expressing an entry in the `to_remove` field in [`DockArea`].
#[derive(Debug, Clone, Copy)]
pub(super) enum TabRemoval {
    Tab(TabPath, ForcedRemoval),
    Node(NodePath),
    Window(SurfaceIndex),
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ForcedRemoval(pub bool);
