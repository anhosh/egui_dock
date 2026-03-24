use egui::Rect;

use crate::{Split, TabIndex};

mod leaf;
mod split;
pub use leaf::LeafNode;
pub use split::SplitNode;

/// Represents an abstract node of a [`Tree`](crate::Tree).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Node<Tab> {
    /// Empty node.
    Empty,

    /// Contains the actual tabs.
    Leaf(LeafNode<Tab>),

    /// Parent node in the vertical orientation.
    Vertical(SplitNode),

    /// Parent node in the horizontal orientation.
    Horizontal(SplitNode),
}

impl<Tab> Node<Tab> {
    /// Constructs a leaf node with a given `tab`.
    #[inline(always)]
    pub fn leaf(tab: Tab) -> Self {
        Self::Leaf(LeafNode::new(vec![tab]))
    }

    /// Get immutable access to the leaf data of this node, if it contains any (i.e is a leaf)
    pub fn get_leaf(&self) -> Option<&LeafNode<Tab>> {
        match self {
            Node::Leaf(leaf_node) => Some(leaf_node),
            _ => None,
        }
    }

    /// Get mutable access to the leaf data of this node, if it contains any (i.e is a leaf)
    pub fn get_leaf_mut(&mut self) -> Option<&mut LeafNode<Tab>> {
        match self {
            Node::Leaf(leaf_node) => Some(leaf_node),
            _ => None,
        }
    }

    /// Constructs a leaf node with a given list of `tabs`.
    #[inline(always)]
    pub const fn leaf_with(tabs: Vec<Tab>) -> Self {
        Self::Leaf(LeafNode::new(tabs))
    }

    /// Sets the area occupied by the node.
    ///
    /// If the node is a ``Node::Empty``, this will do nothing.
    #[inline]
    pub fn set_rect(&mut self, new_rect: Rect) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.set_rect(new_rect),
            Self::Vertical(split) | Self::Horizontal(split) => split.set_rect(new_rect),
        }
    }

    /// Get a [`Rect`] occupied by the node, could be used e.g. to draw a highlight rect around a node.
    ///
    /// Returns [`None`] if node is of the [`Empty`](Node::Empty) variant.
    #[inline]
    pub fn rect(&self) -> Option<Rect> {
        match self {
            Self::Empty => None,
            Self::Leaf(leaf) => Some(leaf.rect()),
            Self::Vertical(split) | Self::Horizontal(split) => Some(split.rect()),
        }
    }

    /// Returns `true` if the node is a [`Empty`](Node::Empty), otherwise `false`.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if the node is a [`Leaf`](Node::Leaf), otherwise `false`.
    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf { .. })
    }

    /// Returns `true` if the node is a [`Horizontal`](Node::Horizontal), otherwise `false`.
    #[inline(always)]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal { .. })
    }

    /// Returns `true` if the node is a [`Vertical`](Node::Vertical), otherwise `false`.
    #[inline(always)]
    pub const fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical { .. })
    }

    /// Returns `true` if the node is either [`Horizontal`](Node::Horizontal) or [`Vertical`](Node::Vertical),
    /// otherwise `false`.
    #[inline(always)]
    pub const fn is_parent(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    /// Returns `true` if the node is collapsed, otherwise `false`.
    #[inline(always)]
    pub fn is_collapsed(&self) -> bool {
        match self {
            Node::Leaf(leaf) => leaf.collapsed,
            Node::Horizontal(split) | Node::Vertical(split) => split.fully_collapsed,
            Node::Empty => false,
        }
    }

    /// Returns the number of layers of collapsed leaf subnodes.
    pub fn collapsed_leaf_count(&self) -> i32 {
        match self {
            Node::Horizontal(split) | Node::Vertical(split) => split.collapsed_leaf_count,
            Node::Leaf(leaf) => {
                if leaf.collapsed {
                    1
                } else {
                    0
                }
            }
            Node::Empty => 0,
        }
    }

    /// Replaces the node with [`Horizontal`](Node::Horizontal) or [`Vertical`](Node::Vertical) (depending on `split`)
    /// and assigns an empty rect to it.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    #[inline]
    pub fn split(&mut self, split: Split, fraction: f32) -> Self {
        assert!((0.0..=1.0).contains(&fraction));
        let rect = Rect::NOTHING;
        let src = match split {
            Split::Left | Split::Right => Node::Horizontal(SplitNode::new(
                rect,
                fraction,
                self.is_collapsed(),
                self.collapsed_leaf_count(),
            )),
            Split::Above | Split::Below => Node::Vertical(SplitNode::new(
                rect,
                fraction,
                self.is_collapsed(),
                self.collapsed_leaf_count(),
            )),
        };
        std::mem::replace(self, src)
    }

    /// Provides an immutable slice of the tabs inside this node.
    ///
    /// Returns [`None`] if the node is not a [`Leaf`](Node::Leaf).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_dock::{DockState, NodeIndex};
    /// let mut dock_state = DockState::new(vec![1, 2, 3, 4, 5, 6]);
    /// assert!(dock_state.main_surface().root_node().unwrap().tabs().unwrap().contains(&4));
    /// ```
    #[inline]
    pub fn tabs(&self) -> Option<&[Tab]> {
        match self {
            Node::Leaf(leaf) => Some(leaf.tabs()),
            _ => None,
        }
    }

    /// Provides an mutable slice of the tabs inside this node.
    ///
    /// Returns [`None`] if the node is not a [`Leaf`](Node::Leaf).
    ///
    /// # Examples
    ///
    /// Modifying tabs inside a node:
    /// ```rust
    /// # use egui_dock::{DockState, NodeIndex};
    /// let mut dock_state = DockState::new(vec![1, 2, 3, 4, 5, 6]);
    /// let mut tabs = dock_state
    ///     .main_surface_mut()
    ///     .root_node_mut()
    ///     .unwrap()
    ///     .tabs_mut()
    ///     .unwrap();
    ///
    /// tabs[0] = 7;
    /// tabs[5] = 8;
    ///
    /// assert_eq!(&tabs, &[7, 2, 3, 4, 5, 8]);
    /// ```
    #[inline]
    pub fn tabs_mut(&mut self) -> Option<&mut [Tab]> {
        match self {
            Node::Leaf(leaf) => Some(leaf.tabs_mut()),
            _ => None,
        }
    }

    /// Returns an [`Iterator`] of tabs in this node.
    ///
    /// If this node is not a [`Leaf`](Self::Leaf), then the returned [`Iterator`] will be empty.
    #[inline]
    pub fn iter_tabs(&self) -> impl Iterator<Item = &Tab> {
        match self.tabs() {
            Some(tabs) => tabs.iter(),
            None => core::slice::Iter::default(),
        }
    }

    /// Returns an [`Iterator`] of tabs in this node with their corresponding [`TabIndex`].
    pub fn iter_tabs_indexed(&self) -> impl Iterator<Item = (TabIndex, &Tab)> {
        self.iter_tabs()
            .enumerate()
            .map(|(index, tab)| (TabIndex(index), tab))
    }

    /// Returns a mutable [`Iterator`] of tabs in this node.
    ///
    /// If this node is not a [`Leaf`](Self::Leaf), then the returned [`Iterator`] will be empty.
    #[inline]
    pub fn iter_tabs_mut(&mut self) -> impl Iterator<Item = &mut Tab> {
        match self.tabs_mut() {
            Some(tabs) => tabs.iter_mut(),
            None => core::slice::IterMut::default(),
        }
    }

    /// Returns a mutable [`Iterator`] of tabs in this node with their corresponding [`TabIndex`].
    pub fn iter_tabs_mut_indexed(&mut self) -> impl Iterator<Item = (TabIndex, &mut Tab)> {
        self.iter_tabs_mut()
            .enumerate()
            .map(|(index, tab)| (TabIndex(index), tab))
    }

    /// Adds `tab` to the node and sets it as the active tab.
    ///
    /// # Panics
    ///
    /// If the new capacity of `tabs` exceeds `isize::MAX` bytes.
    ///
    /// If `self` is not a [`Leaf`](Node::Leaf) node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_dock::{DockState, NodeIndex};
    /// let mut dock_state = DockState::new(vec!["a tab"]);
    /// assert_eq!(dock_state.main_surface().root_node().unwrap().tabs_count(), 1);
    ///
    /// dock_state.main_surface_mut().root_node_mut().unwrap().append_tab("another tab");
    /// assert_eq!(dock_state.main_surface().root_node().unwrap().tabs_count(), 2);
    /// ```
    #[track_caller]
    #[inline]
    pub fn append_tab(&mut self, tab: Tab) {
        match self {
            Node::Leaf(leaf) => leaf.append_tab(tab),
            _ => panic!("node was not a leaf"),
        }
    }

    /// Sets the collapsing state of the node.
    ///
    /// # Panics
    ///
    /// Panics if `self` is an [`Empty`](Node::Empty) node.
    #[inline]
    pub fn set_collapsed(&mut self, collapsed: bool) {
        match self {
            Node::Leaf(leaf) => leaf.collapsed = collapsed,
            Node::Vertical(split) | Node::Horizontal(split) => split.fully_collapsed = collapsed,
            Node::Empty => panic!("node was empty"),
        }
    }

    /// Sets the number of layers of collapsed leaf subnodes.
    ///
    /// # Panics
    ///
    /// Panics if `self` is neither a [`Vertical`](Node::Vertical) nor a [`Horizontal`](Node::Horizontal) node.
    #[inline]
    pub fn set_collapsed_leaf_count(&mut self, count: i32) {
        match self {
            Node::Horizontal(split) | Node::Vertical(split) => split.collapsed_leaf_count = count,
            _ => panic!("node was neither vertical nor horizontal"),
        }
    }

    /// Adds a `tab` to the node.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity of `tabs` exceeds `isize::MAX` bytes, or `index > tabs_count()`.
    #[track_caller]
    #[inline]
    pub fn insert_tab(&mut self, index: TabIndex, tab: Tab) {
        match self {
            Node::Leaf(leaf) => leaf.insert_tab(index, tab),
            _ => panic!("node was not a leaf!"),
        }
    }

    /// Removes a tab at given `index` from the node.
    /// Returns the removed tab if the node is a `Leaf`, or `None` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn remove_tab(&mut self, tab_index: TabIndex) -> Option<Tab> {
        match self {
            Node::Leaf(leaf) => leaf.remove_tab(tab_index),
            _ => None,
        }
    }

    /// Gets the number of tabs in the node.
    #[inline]
    pub fn tabs_count(&self) -> usize {
        match self {
            Node::Leaf(leaf) => leaf.tabs.len(),
            _ => Default::default(),
        }
    }

    /// Returns a new [`Node`] while mapping and filtering the tab type.
    /// If this [`Node`] remains empty, it will change to [`Node::Empty`].
    pub fn filter_map_tabs<F, NewTab>(&self, function: F) -> Node<NewTab>
    where
        F: FnMut(&Tab) -> Option<NewTab>,
    {
        match self {
            Node::Leaf(leaf) => {
                let LeafNode {
                    rect,
                    viewport,
                    tabs,
                    active,
                    scroll,
                    collapsed,
                } = leaf;
                let tabs: Vec<_> = tabs.iter().filter_map(function).collect();
                if tabs.is_empty() {
                    Node::Empty
                } else {
                    Node::Leaf(LeafNode {
                        rect: *rect,
                        viewport: *viewport,
                        tabs,
                        active: *active,
                        scroll: *scroll,
                        collapsed: *collapsed,
                    })
                }
            }
            Node::Empty => Node::Empty,
            Node::Vertical(split) => Node::Vertical(split.clone()),
            Node::Horizontal(split) => Node::Horizontal(split.clone()),
        }
    }

    /// Returns a new [`Node`] while mapping the tab type.
    pub fn map_tabs<F, NewTab>(&self, mut function: F) -> Node<NewTab>
    where
        F: FnMut(&Tab) -> NewTab,
    {
        self.filter_map_tabs(move |tab| Some(function(tab)))
    }

    /// Returns a new [`Node`] while filtering the tab type.
    /// If this [`Node`] remains empty, it will change to [`Node::Empty`].
    pub fn filter_tabs<F>(&self, mut predicate: F) -> Node<Tab>
    where
        F: FnMut(&Tab) -> bool,
        Tab: Clone,
    {
        self.filter_map_tabs(move |tab| predicate(tab).then(|| tab.clone()))
    }

    /// Removes all tabs for which `predicate` returns `false`.
    /// If this [`Node`] remains empty, it will change to [`Node::Empty`].
    pub fn retain_tabs<F>(&mut self, predicate: F)
    where
        F: FnMut(&mut Tab) -> bool,
    {
        if let Node::Leaf(leaf) = self {
            leaf.retain_tabs(predicate);
            if leaf.tabs.is_empty() {
                *self = Node::Empty;
            }
        }
    }
}
