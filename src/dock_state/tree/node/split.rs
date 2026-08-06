use egui::Rect;

/// The inner data of a [``Node::Horizontal``](crate::Node)/[``Node::Vertical``](crate::Node), which splits into two further nodes.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct SplitNode {
    /// The rectangle in which all children of this node are drawn.
    pub rect: Rect,

    /// The fraction taken by the top child of this node.
    pub fraction: f32,

    /// Whether all subnodes are collapsed.
    pub fully_collapsed: bool,

    /// The number of collapsed leaf subnodes.
    pub collapsed_leaf_count: i32,
}

impl SplitNode {
    /// Create a new ``SplitNode``
    pub const fn new(
        rect: Rect,
        fraction: f32,
        fully_collapsed: bool,
        collapsed_leaf_count: i32,
    ) -> Self {
        Self {
            rect,
            fraction,
            fully_collapsed,
            collapsed_leaf_count,
        }
    }
    /// Set the Area which this ``SplitNode`` occupies.
    #[inline]
    pub fn set_rect(&mut self, new_rect: Rect) {
        self.rect = new_rect;
    }

    /// Get the Area which this ``SplitNode`` occupies.
    pub fn rect(&self) -> Rect {
        self.rect
    }
}

impl Default for SplitNode {
    fn default() -> Self {
        Self::new(Rect::NOTHING, 0.0, false, 0)
    }
}
