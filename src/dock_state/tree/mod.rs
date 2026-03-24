//! Binary tree representing the relationships between [`Node`]s.
//!
//! # Implementation details
//!
//! The binary tree is stored in a [`Vec`] indexed by [`NodeIndex`].
//! The root is always at index *0*.
//! For a given node *n*:
//!  - left child of *n* will be at index *n * 2 + 1*.
//!  - right child of *n* will be at index *n * 2 + 2*.

/// Iterates over all tabs in a [`Tree`].
pub mod tab_iter;

/// Identifies a tab within a [`Node`].
pub mod tab_index;

/// Represents an abstract node of a [`Tree`].
pub mod node;

/// Wrapper around indices to the collection of nodes inside a [`Tree`].
pub mod node_index;

use std::{
    cmp::max,
    fmt,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use egui::ahash::HashSet;
use egui::Rect;
pub use node::LeafNode;
pub use node::Node;
pub use node::SplitNode;
pub use node_index::{NodeIndex, NodePath};
pub use tab_index::{TabIndex, TabPath};
pub use tab_iter::TabIter;

use crate::{Error, Result, SurfaceIndex};

// ----------------------------------------------------------------------------

/// Direction in which a new node is created relatively to the parent node at which the split occurs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(missing_docs)]
pub enum Split {
    Left,
    Right,
    Above,
    Below,
}

impl Split {
    /// Returns whether the split is vertical.
    pub const fn is_top_bottom(self) -> bool {
        matches!(self, Split::Above | Split::Below)
    }

    /// Returns whether the split is horizontal.
    pub const fn is_left_right(self) -> bool {
        matches!(self, Split::Left | Split::Right)
    }
}

/// Specify how a tab should be added to a Node.
pub enum TabInsert {
    /// Split the node in the given direction.
    Split(Split),

    /// Insert the tab at the given index.
    Insert(TabIndex),

    /// Append the tab to the node.
    Append,
}

/// The destination for a tab which is being moved.
pub enum TabDestination {
    /// Move to a new window with this rect.
    Window(Rect),

    /// Move to an existing node with this insertion.
    Node(NodePath, TabInsert),

    /// Move to an empty surface.
    EmptySurface(SurfaceIndex),
}

impl From<(NodePath, TabInsert)> for TabDestination {
    fn from(value: (NodePath, TabInsert)) -> TabDestination {
        TabDestination::Node(value.0, value.1)
    }
}

impl From<SurfaceIndex> for TabDestination {
    fn from(value: SurfaceIndex) -> TabDestination {
        TabDestination::EmptySurface(value)
    }
}

impl TabDestination {
    /// Returns if this tab destination is a [`Window`](TabDestination::Window).
    pub fn is_window(&self) -> bool {
        matches!(self, Self::Window(_))
    }
}

/// Binary tree representing the relationships between [`Node`]s.
///
/// # Implementation details
///
/// The binary tree is stored in a [`Vec`] indexed by [`NodeIndex`].
/// The root is always at index *0*.
/// For a given node *n*:
///  - left child of *n* will be at index *n * 2 + 1*.
///  - right child of *n* will be at index *n * 2 + 2*.
///
/// For "Horizontal" nodes:
///  - left child contains Left node.
///  - right child contains Right node.
///
/// For "Vertical" nodes:
///  - left child contains Top node.
///  - right child contains Bottom node.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree<Tab> {
    // Binary tree vector
    pub(super) nodes: Vec<Node<Tab>>,
    focused_node: Option<NodeIndex>,
    // Whether all subnodes of the tree is collapsed
    collapsed: bool,
    collapsed_leaf_count: i32,
}

impl<Tab> fmt::Debug for Tree<Tab> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tree").finish_non_exhaustive()
    }
}

impl<Tab> Default for Tree<Tab> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            focused_node: None,
            collapsed: false,
            collapsed_leaf_count: 0,
        }
    }
}

impl<Tab> Index<NodeIndex> for Tree<Tab> {
    type Output = Node<Tab>;

    #[inline(always)]
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl<Tab> IndexMut<NodeIndex> for Tree<Tab> {
    #[inline(always)]
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[index.0]
    }
}

impl<Tab> Tree<Tab> {
    /// Creates a new [`Tree`] with given `Vec` of `Tab`s in its root node.
    #[inline(always)]
    pub fn new(tabs: Vec<Tab>) -> Self {
        let root = Node::leaf_with(tabs);
        Self {
            nodes: vec![root],
            focused_node: None,
            collapsed: false,
            collapsed_leaf_count: 0,
        }
    }

    /// Returns the viewport [`Rect`] and the `Tab` inside the first leaf node,
    /// or `None` if no leaf exists in the [`Tree`].
    #[inline]
    pub fn find_active(&mut self) -> Option<(Rect, &mut Tab)> {
        self.nodes.iter_mut().find_map(|node| match node {
            Node::Leaf(leaf) => leaf
                .tabs
                .get_mut(leaf.active.0)
                .map(|tab| (leaf.viewport.to_owned(), tab)),
            _ => None,
        })
    }

    /// Returns the number of nodes in the [`Tree`].
    ///
    /// This includes [`Empty`](Node::Empty) nodes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the number of nodes in the tree is 0, otherwise `false`.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns an [`Iterator`] of the underlying collection of nodes.
    ///
    /// This includes [`Empty`](Node::Empty) nodes.
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, Node<Tab>> {
        self.nodes.iter()
    }

    /// Returns [`IterMut`] of the underlying collection of nodes.
    ///
    /// This includes [`Empty`](Node::Empty) nodes.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, Node<Tab>> {
        self.nodes.iter_mut()
    }

    /// Returns an [`Iterator`] of [`NodeIndex`] ordered in a breadth first manner.
    #[inline(always)]
    pub(crate) fn breadth_first_index_iter(&self) -> impl Iterator<Item = NodeIndex> {
        (0..self.nodes.len()).map(NodeIndex)
    }

    /// Returns an iterator over all tabs in arbitrary order.
    #[inline(always)]
    pub fn tabs(&self) -> TabIter<'_, Tab> {
        TabIter::new(self)
    }

    /// Counts and returns the number of tabs in the whole tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_dock::{DockState, NodeIndex, TabIndex};
    /// let mut dock_state = DockState::new(vec!["node 1", "node 2", "node 3"]);
    /// assert_eq!(dock_state.main_surface().num_tabs(), 3);
    ///
    /// let [a, b] = dock_state.main_surface_mut().split_left(NodeIndex::root(), 0.5, vec!["tab 4", "tab 5"]);
    /// assert_eq!(dock_state.main_surface().num_tabs(), 5);
    ///
    /// dock_state.main_surface_mut().remove_leaf(a);
    /// assert_eq!(dock_state.main_surface().num_tabs(), 2);
    /// ```
    #[inline]
    pub fn num_tabs(&self) -> usize {
        let mut count = 0;
        for node in self.nodes.iter() {
            if let Node::Leaf(leaf) = node {
                count += leaf.tabs.len();
            }
        }
        count
    }

    /// Acquire a immutable borrow to the [`Node`] at the root of the tree.
    /// Returns [`None`] if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_dock::DockState;
    /// let mut dock_state = DockState::new(vec!["single tab"]);
    /// let root_node = dock_state.main_surface().root_node().unwrap();
    ///
    /// assert_eq!(root_node.tabs(), Some(["single tab"].as_slice()));
    /// ```
    pub fn root_node(&self) -> Option<&Node<Tab>> {
        self.nodes.first()
    }

    /// Acquire a mutable borrow to the [`Node`] at the root of the tree.
    /// Returns [`None`] if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_dock::{DockState, LeafNode};
    /// let mut dock_state = DockState::new(vec!["single tab"]);
    /// let root_node = dock_state.main_surface_mut().root_node_mut().unwrap();
    /// let root_as_leaf = root_node.get_leaf_mut().unwrap();
    /// root_as_leaf.tabs.push("partner tab");
    ///
    /// assert_eq!(root_node.tabs(), Some(["single tab", "partner tab"].as_slice()));
    /// ```
    pub fn root_node_mut(&mut self) -> Option<&mut Node<Tab>> {
        self.nodes.first_mut()
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) gets the `tabs`.
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed relatively to the old node, in the direction specified by `split`.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use egui_dock::{DockState, SurfaceIndex, NodeIndex, Split};
    /// let mut dock_state = DockState::new(vec!["tab 1", "tab 2"]);
    ///
    /// // At this point, the main surface only contains the leaf with tab 1 and 2.
    /// assert!(dock_state.main_surface().root_node().unwrap().is_leaf());
    ///
    /// // Split the node, giving 50% of the space to the new nodes and 50% to the old ones.
    /// let [old, new] = dock_state.main_surface_mut()
    ///     .split_tabs(NodeIndex::root(), Split::Below, 0.5, vec!["tab 3"]);
    ///
    /// assert!(dock_state.main_surface().root_node().unwrap().is_parent());
    /// assert!(dock_state[SurfaceIndex::main()][old].is_leaf());
    /// assert!(dock_state[SurfaceIndex::main()][new].is_leaf());
    /// ```
    #[inline(always)]
    pub fn split_tabs(
        &mut self,
        parent: NodeIndex,
        split: Split,
        fraction: f32,
        tabs: Vec<Tab>,
    ) -> [NodeIndex; 2] {
        self.split(parent, split, fraction, Node::leaf_with(tabs))
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) gets the `tabs`.
    ///
    /// This is a shorthand for using `split_tabs` with [`Split::Above`].
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed *above* the old node.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    #[inline(always)]
    pub fn split_above(
        &mut self,
        parent: NodeIndex,
        fraction: f32,
        tabs: Vec<Tab>,
    ) -> [NodeIndex; 2] {
        self.split(parent, Split::Above, fraction, Node::leaf_with(tabs))
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) gets the `tabs`.
    ///
    /// This is a shorthand for using `split_tabs` with [`Split::Below`].
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed *below* the old node.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    #[inline(always)]
    pub fn split_below(
        &mut self,
        parent: NodeIndex,
        fraction: f32,
        tabs: Vec<Tab>,
    ) -> [NodeIndex; 2] {
        self.split(parent, Split::Below, fraction, Node::leaf_with(tabs))
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) gets the `tabs`.
    ///
    /// This is a shorthand for using `split_tabs` with [`Split::Left`].
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed to the *left* of the old node.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    #[inline(always)]
    pub fn split_left(
        &mut self,
        parent: NodeIndex,
        fraction: f32,
        tabs: Vec<Tab>,
    ) -> [NodeIndex; 2] {
        self.split(parent, Split::Left, fraction, Node::leaf_with(tabs))
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) gets the `tabs`.
    ///
    /// This is a shorthand for using `split_tabs` with [`Split::Right`].
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed to the *right* of the old node.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    #[inline(always)]
    pub fn split_right(
        &mut self,
        parent: NodeIndex,
        fraction: f32,
        tabs: Vec<Tab>,
    ) -> [NodeIndex; 2] {
        self.split(parent, Split::Right, fraction, Node::leaf_with(tabs))
    }

    /// Creates two new nodes by splitting a given `parent` node and assigns them as its children. The first (old) node
    /// inherits content of the `parent` from before the split, and the second (new) uses `new`.
    ///
    /// `fraction` (in range 0..=1) specifies how much of the `parent` node's area the old node will attempt to occupy
    /// after the split.
    ///
    /// The new node is placed relatively to the old node, in the direction specified by `split`.
    ///
    /// Returns the indices of the old node and the new node.
    ///
    /// # Panics
    ///
    /// If `fraction` isn't in range 0..=1.
    ///
    /// If `new` is an [`Empty`](Node::Empty), [`Horizontal`](Node::Horizontal) or [`Vertical`](Node::Vertical) node.
    ///
    /// If `new` is a [`Leaf`](Node::Leaf) node without any tabs.
    ///
    /// If `parent` points to an [`Empty`](Node::Empty) node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use egui_dock::{DockState, SurfaceIndex, NodeIndex, Split, Node};
    /// let mut dock_state = DockState::new(vec!["tab 1", "tab 2"]);
    ///
    /// // At this point, the main surface only contains the leaf with tab 1 and 2.
    /// assert!(dock_state.main_surface().root_node().unwrap().is_leaf());
    ///
    /// // Splits the node, giving 50% of the space to the new nodes and 50% to the old ones.
    /// let [old, new] = dock_state.main_surface_mut()
    ///     .split(NodeIndex::root(), Split::Below, 0.5, Node::leaf_with(vec!["tab 3"]));
    ///
    /// assert!(dock_state.main_surface().root_node().unwrap().is_parent());
    /// assert!(dock_state[SurfaceIndex::main()][old].is_leaf());
    /// assert!(dock_state[SurfaceIndex::main()][new].is_leaf());
    /// ```
    pub fn split(
        &mut self,
        parent: NodeIndex,
        split: Split,
        fraction: f32,
        new: Node<Tab>,
    ) -> [NodeIndex; 2] {
        let old = self[parent].split(split, fraction);
        assert!(old.is_leaf() || old.is_parent());
        assert_ne!(new.tabs_count(), 0);
        // Resize vector to fit the new size of the binary tree.
        {
            let index = self.nodes.iter().rposition(|n| !n.is_empty()).unwrap_or(0);
            let level = NodeIndex(index).level();
            self.nodes
                .resize_with((1 << (level + 1)) - 1, || Node::Empty);
        }

        let index = match split {
            Split::Left | Split::Above => [parent.right(), parent.left()],
            Split::Right | Split::Below => [parent.left(), parent.right()],
        };

        // If the node were splitting is a parent, all it's children need to be moved.
        if old.is_parent() {
            let levels_to_move = NodeIndex(self.nodes.len()).level() - index[0].level();

            // Level 0 is ourself, which is done when we assign self[index[0]] = old, so start at 1.
            for level in (1..levels_to_move).rev() {
                // Old child indices for this level
                let old_start = parent.children_at(level).start;
                // New child indices for this level
                let new_start = index[0].children_at(level).start;

                // Children to be moved this level change
                let len = 1 << level;

                // Swap self[old_start..(old_start+len)] with self[new_start..(new_start+len)]
                // (the new part will only contain empty entries).
                let (old_range, new_range) = {
                    let (first_part, second_part) = self.nodes.split_at_mut(new_start);
                    // Cut to length.
                    (
                        &mut first_part[old_start..old_start + len],
                        &mut second_part[..len],
                    )
                };
                old_range.swap_with_slice(new_range);
            }
        }

        self[index[0]] = old;
        self[index[1]] = new;

        self.focused_node = Some(index[1]);
        self.node_update_collapsed(index[1]);

        index
    }

    /// Immutably borrows a leaf node at the given path.
    ///
    /// Returns `Err` if the path is invalid or the node at the path is not a leaf.
    pub fn leaf(&self, node: NodeIndex) -> Result<&LeafNode<Tab>> {
        self.nodes
            .get(node.0)
            .ok_or(Error::InvalidNode)?
            .get_leaf()
            .ok_or(Error::NonLeafNode)
    }

    /// Mutably borrows a leaf node at the given index.
    ///
    /// Returns `Err` if the index is invalid or if the node is not a leaf.
    pub fn leaf_mut(&mut self, node: NodeIndex) -> Result<&mut LeafNode<Tab>> {
        self.nodes
            .get_mut(node.0)
            .ok_or(Error::InvalidNode)?
            .get_leaf_mut()
            .ok_or(Error::NonLeafNode)
    }

    fn first_leaf(&self, top: NodeIndex) -> Option<NodeIndex> {
        let left = top.left();
        let right = top.right();
        match (self.nodes.get(left.0), self.nodes.get(right.0)) {
            (Some(&Node::Leaf { .. }), _) => Some(left),
            (_, Some(&Node::Leaf { .. })) => Some(right),

            (
                Some(Node::Horizontal { .. } | Node::Vertical { .. }),
                Some(Node::Horizontal { .. } | Node::Vertical { .. }),
            ) => self.first_leaf(left).or(self.first_leaf(right)),
            (Some(Node::Horizontal { .. } | Node::Vertical { .. }), _) => self.first_leaf(left),
            (_, Some(Node::Horizontal { .. } | Node::Vertical { .. })) => self.first_leaf(right),

            (None, None)
            | (Some(&Node::Empty), None)
            | (None, Some(&Node::Empty))
            | (Some(&Node::Empty), Some(&Node::Empty)) => None,
        }
    }

    /// Returns the viewport [`Rect`] and the `Tab` inside the focused leaf node or [`None`] if it does not exist.
    #[inline]
    pub fn find_active_focused(&mut self) -> Option<(Rect, &mut Tab)> {
        match self.focused_node.and_then(|idx| self.nodes.get_mut(idx.0)) {
            Some(Node::Leaf(leaf)) => leaf.active_focused(),
            _ => None,
        }
    }

    /// Gets the node index of currently focused leaf node; returns [`None`] when no leaf is focused.
    #[inline]
    pub fn focused_leaf(&self) -> Option<NodeIndex> {
        self.focused_node
    }

    /// Sets the currently focused leaf to `node_index` if the node at `node_index` is a leaf.
    ///
    /// This method will not never panic and instead removes focus from all nodes when given an invalid index.
    #[inline]
    pub fn set_focused_node(&mut self, node_index: NodeIndex) {
        self.focused_node = self
            .nodes
            .get(node_index.0)
            .filter(|node| node.is_leaf())
            .map(|_| node_index);
    }

    /// Removes the given node from the [`Tree`].
    ///
    /// # Panics
    ///
    /// - If the tree is empty.
    /// - If the node at index `node` is not a [`Leaf`](Node::Leaf).
    pub fn remove_leaf(&mut self, node: NodeIndex) {
        assert!(!self.is_empty());
        assert!(self[node].is_leaf());

        let Some(parent) = node.parent() else {
            self.nodes.clear();
            return;
        };

        if Some(node) == self.focused_node {
            self.focused_node = None;
            let mut node = node;
            while let Some(parent) = node.parent() {
                let next = if node.is_left() {
                    parent.right()
                } else {
                    parent.left()
                };
                if self.nodes.get(next.0).is_some_and(|node| node.is_leaf()) {
                    self.focused_node = Some(next);
                    break;
                }
                if let Some(node) = self.first_leaf(next) {
                    self.focused_node = Some(node);
                    break;
                }
                node = parent;
            }
        }

        self[parent] = Node::Empty;
        self[node] = Node::Empty;

        let mut level = 0;

        if node.is_left() {
            'left_end: loop {
                let dst = parent.children_at(level);
                let src = parent.children_right(level + 1);
                for (dst, src) in dst.zip(src) {
                    if src >= self.nodes.len() {
                        break 'left_end;
                    }
                    if Some(NodeIndex(src)) == self.focused_node {
                        self.focused_node = Some(NodeIndex(dst));
                    }
                    self.nodes[dst] = std::mem::replace(&mut self.nodes[src], Node::Empty);
                }
                level += 1;
            }
        } else {
            'right_end: loop {
                let dst = parent.children_at(level);
                let src = parent.children_left(level + 1);
                for (dst, src) in dst.zip(src) {
                    if src >= self.nodes.len() {
                        break 'right_end;
                    }
                    if Some(NodeIndex(src)) == self.focused_node {
                        self.focused_node = Some(NodeIndex(dst));
                    }
                    self.nodes[dst] = std::mem::replace(&mut self.nodes[src], Node::Empty);
                }
                level += 1;
            }
        }
        // Ensure that there are no trailing `Node::Empty` items
        while let Some(last_index) = self.nodes.len().checked_sub(1).map(NodeIndex) {
            if self[last_index].is_empty()
                && last_index.parent().is_some_and(|pi| !self[pi].is_parent())
            {
                self.nodes.pop();
            } else {
                break;
            }
        }
    }

    /// Pushes a tab to the first `Leaf` it finds or create a new leaf if an `Empty` node is encountered.
    pub fn push_to_first_leaf(&mut self, tab: Tab) {
        for (index, node) in &mut self.nodes.iter_mut().enumerate() {
            match node {
                Node::Leaf(leaf) => {
                    leaf.active = TabIndex(leaf.tabs.len());
                    leaf.tabs.push(tab);
                    self.focused_node = Some(NodeIndex(index));
                    return;
                }
                Node::Empty => {
                    *node = Node::leaf(tab);
                    self.focused_node = Some(NodeIndex(index));
                    return;
                }
                _ => {}
            }
        }
        assert!(self.nodes.is_empty());
        self.nodes.push(Node::leaf_with(vec![tab]));
        self.focused_node = Some(NodeIndex(0));
    }

    /// Sets which is the active tab within a specific node.
    ///
    /// # Errors
    /// If the node is invalid, not a leaf or if the tab index is out of bounds.
    #[inline]
    pub fn set_active_tab(
        &mut self,
        node_index: impl Into<NodeIndex>,
        tab_index: impl Into<TabIndex>,
    ) -> Result {
        self.leaf_mut(node_index.into())?
            .set_active_tab(tab_index.into())
    }

    /// Pushes `tab` to the currently focused leaf.
    ///
    /// If no leaf is focused it will be pushed to the first available leaf.
    ///
    /// If no leaf is available then a new leaf will be created.
    pub fn push_to_focused_leaf(&mut self, tab: Tab) {
        match self.focused_node {
            Some(node) => {
                if self.nodes.is_empty() {
                    self.nodes.push(Node::leaf(tab));
                    self.focused_node = Some(NodeIndex::root());
                } else {
                    match &mut self[node] {
                        Node::Empty => {
                            self[node] = Node::leaf(tab);
                            self.focused_node = Some(node);
                        }
                        Node::Leaf(leaf) => {
                            leaf.append_tab(tab);
                            self.focused_node = Some(node);
                        }
                        _ => {
                            self.push_to_first_leaf(tab);
                        }
                    }
                }
            }
            None => {
                if self.nodes.is_empty() {
                    self.nodes.push(Node::leaf(tab));
                    self.focused_node = Some(NodeIndex::root());
                } else {
                    self.push_to_first_leaf(tab);
                }
            }
        }
    }

    /// Removes the tab at the given ([`NodeIndex`], [`TabIndex`]) pair.
    ///
    /// If the node is emptied after the tab is removed, the node will also be removed.
    ///
    /// Returns the removed tab if it exists, or `None` otherwise.
    pub fn remove_tab(&mut self, (node_index, tab_index): (NodeIndex, TabIndex)) -> Option<Tab> {
        let node = &mut self[node_index];
        let tab = node.remove_tab(tab_index);
        if node.tabs_count() == 0 {
            self.remove_leaf(node_index);
        }
        tab
    }

    /// Returns a new [`Tree`] while mapping and filtering the tab type.
    /// Any remaining empty [`Node`]s are removed.
    pub fn filter_map_tabs<F, NewTab>(&self, mut function: F) -> Tree<NewTab>
    where
        F: FnMut(&Tab) -> Option<NewTab>,
    {
        let Tree {
            focused_node,
            nodes,
            collapsed,
            collapsed_leaf_count,
        } = self;
        let mut emptied_nodes = HashSet::default();
        let nodes = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let filtered_node = node.filter_map_tabs(&mut function);
                if filtered_node.is_empty() && !node.is_empty() {
                    emptied_nodes.insert(NodeIndex(index));
                }
                filtered_node
            })
            .collect();
        let mut new_tree = Tree {
            nodes,
            focused_node: *focused_node,
            collapsed: *collapsed,
            collapsed_leaf_count: *collapsed_leaf_count,
        };
        new_tree.balance(emptied_nodes);
        new_tree
    }

    /// Returns a new [`Tree`] while mapping the tab type.
    pub fn map_tabs<F, NewTab>(&self, mut function: F) -> Tree<NewTab>
    where
        F: FnMut(&Tab) -> NewTab,
    {
        self.filter_map_tabs(move |tab| Some(function(tab)))
    }

    /// Returns a new [`Tree`] while filtering the tab type.
    /// Any remaining empty [`Node`]s are removed.
    pub fn filter_tabs<F>(&self, mut predicate: F) -> Tree<Tab>
    where
        F: FnMut(&Tab) -> bool,
        Tab: Clone,
    {
        self.filter_map_tabs(move |tab| predicate(tab).then(|| tab.clone()))
    }

    /// Removes all tabs for which `predicate` returns `false`.
    /// Any remaining empty [`Node`]s are also removed.
    pub fn retain_tabs<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&mut Tab) -> bool,
    {
        let mut emptied_nodes = HashSet::default();
        for (index, node) in self.nodes.iter_mut().enumerate() {
            node.retain_tabs(&mut predicate);
            if node.is_empty() {
                emptied_nodes.insert(NodeIndex(index));
            }
        }
        self.balance(emptied_nodes);
    }

    /// Sets the collapsing state of the [`Tree`].
    pub(crate) fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
    }

    /// Returns whether the [`Tree`] is collapsed.
    pub(crate) fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Sets the number of collapsed layers of leaf subnodes in the [`Tree`].
    pub(crate) fn set_collapsed_leaf_count(&mut self, collapsed_leaf_count: i32) {
        self.collapsed_leaf_count = collapsed_leaf_count;
    }

    /// Returns the number of collapsed layers of leaf subnodes in the [`Tree`].
    pub(crate) fn collapsed_leaf_count(&self) -> i32 {
        self.collapsed_leaf_count
    }

    fn balance(&mut self, emptied_nodes: HashSet<NodeIndex>) {
        let mut emptied_parents = HashSet::default();
        for parent_index in emptied_nodes.into_iter().filter_map(|ni| ni.parent()) {
            if !self[parent_index].is_parent() {
                continue;
            } else if self[parent_index.left()].is_empty() && self[parent_index.right()].is_empty()
            {
                self[parent_index] = Node::Empty;
                emptied_parents.insert(parent_index);
            } else if self[parent_index.left()].is_empty() {
                self.nodes.swap(parent_index.0, parent_index.right().0);
                self[parent_index.right()] = Node::Empty;
            } else if self[parent_index.right()].is_empty() {
                self.nodes.swap(parent_index.0, parent_index.left().0);
                self[parent_index.left()] = Node::Empty;
            }
        }
        if !emptied_parents.is_empty() {
            self.balance(emptied_parents);
        }
    }

    /// Updates the collapsed state of the node and its parents.
    pub(crate) fn node_update_collapsed(&mut self, node_index: NodeIndex) {
        let collapsed = self[node_index].is_collapsed();
        if !collapsed {
            // Recursively notify parent nodes that the leaf has expanded
            let mut parent_index_option = node_index.parent();
            while let Some(parent_index) = parent_index_option {
                parent_index_option = parent_index.parent();

                // Update collapsed leaf count and collapse status
                let left_count = self[parent_index.left()].collapsed_leaf_count();
                let right_count = self[parent_index.right()].collapsed_leaf_count();
                self[parent_index].set_collapsed(false);

                if self[parent_index].is_horizontal() {
                    self[parent_index].set_collapsed_leaf_count(max(left_count, right_count));
                } else {
                    self[parent_index].set_collapsed_leaf_count(left_count + right_count);
                }
            }
            self.set_collapsed(false);
            let root_index = NodeIndex::root();
            self.set_collapsed_leaf_count(self[root_index].collapsed_leaf_count());
        } else {
            // Recursively notify parent nodes that the leaf has collapsed
            let mut parent_index_option = node_index.parent();
            while let Some(parent_index) = parent_index_option {
                parent_index_option = parent_index.parent();

                // Update collapsed leaf count and collapse status
                let left_count = self[parent_index.left()].collapsed_leaf_count();
                let right_count = self[parent_index.right()].collapsed_leaf_count();

                if self[parent_index].is_horizontal() {
                    self[parent_index].set_collapsed_leaf_count(max(left_count, right_count));
                } else {
                    self[parent_index].set_collapsed_leaf_count(left_count + right_count);
                }

                if self[parent_index.left()].is_collapsed()
                    && self[parent_index.right()].is_collapsed()
                {
                    self[parent_index].set_collapsed(true);
                }
            }
            if self.root_node().is_some_and(|root| root.is_collapsed()) {
                self.set_collapsed(true);
                let root_index = NodeIndex::root();
                self.set_collapsed_leaf_count(self[root_index].collapsed_leaf_count());
            }
        }
    }

    /// Find a given tab based on ``predicate``.
    ///
    /// Returns the indices in where that node and tab is in this surface.
    ///
    /// The returned [`NodeIndex`] will always point to a [`Node::Leaf`].
    ///
    /// In case there are several hits, only the first is returned.
    pub fn find_tab_from(&self, predicate: impl Fn(&Tab) -> bool) -> Option<(NodeIndex, TabIndex)> {
        for (node_index, node) in self.nodes.iter().enumerate() {
            if let Some(tabs) = node.tabs() {
                for (tab_index, tab) in tabs.iter().enumerate() {
                    if predicate(tab) {
                        return Some((node_index.into(), tab_index.into()));
                    }
                }
            };
        }
        None
    }
}

impl<Tab> Tree<Tab>
where
    Tab: PartialEq,
{
    /// Find the given tab.
    ///
    /// Returns in which node and where in that node the tab is.
    ///
    /// The returned [`NodeIndex`] will always point to a [`Node::Leaf`].
    ///
    /// In case there are several hits, only the first is returned.
    pub fn find_tab(&self, needle_tab: &Tab) -> Option<(NodeIndex, TabIndex)> {
        self.find_tab_from(|tab| tab == needle_tab)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Tab(u64);

    /// Checks that `retain` works after removing a node
    #[test]
    fn remove_and_retain() {
        let mut tree: Tree<Tab> = Tree::new(vec![]);
        tree.push_to_focused_leaf(Tab(0));
        let (n0, _t0) = tree.find_tab(&Tab(0)).unwrap();
        tree.split_below(n0, 0.5, vec![Tab(1)]);

        let i1 = tree.find_tab(&Tab(1)).unwrap();
        tree.remove_tab(i1);
        assert_eq!(tree.nodes.len(), 1);

        tree.retain_tabs(|_| true);
        assert!(tree.find_tab(&Tab(0)).is_some());
    }

    /// Tests whether `retain_tabs` works correctly with trailing `Empty` nodes
    #[test]
    fn retain_trailing_empty() {
        let mut tree: Tree<Tab> = Tree::new(vec![]);
        tree.push_to_focused_leaf(Tab(0));
        tree.nodes.push(Node::Empty);
        tree.nodes.push(Node::Empty);

        tree.retain_tabs(|_| true);
        assert!(tree.find_tab(&Tab(0)).is_some());
    }
}
