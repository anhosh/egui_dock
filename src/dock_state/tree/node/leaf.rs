use std::ops;

use egui::Rect;

use crate::{Error, Result, TabIndex};

/// The inner data of a [``Node::Leaf``](crate::Node), which contains tabs and can be collapsed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LeafNode<Tab> {
    /// The full rectangle - tab bar plus tab body.
    pub rect: Rect,

    /// The tab body rectangle.
    pub viewport: Rect,

    /// All the tabs in this node.
    pub tabs: Vec<Tab>,

    /// The opened tab.
    pub active: TabIndex,

    /// Scroll amount of the tab bar.
    pub scroll: f32,

    /// Whether the leaf is collapsed.
    pub collapsed: bool,
}

impl<Tab> LeafNode<Tab> {
    /// Create New LeafNode with specified ``tabs``, all other internal values will be filled by "nothing" defaults.
    pub const fn new(tabs: Vec<Tab>) -> Self {
        LeafNode {
            rect: Rect::NOTHING,
            viewport: Rect::NOTHING,
            tabs,
            active: TabIndex(0),
            scroll: 0.0,
            collapsed: false,
        }
    }

    /// Set the active tab of this [`LeafNode`]
    ///
    /// If `active_tab` is out of bounds, an error is returned and the active tab is not changed.
    #[inline]
    pub fn set_active_tab(&mut self, active_tab: impl Into<TabIndex>) -> Result {
        let index = active_tab.into();
        if index.0 < self.len() {
            self.active = index;
            Ok(())
        } else {
            Err(Error::InvalidTab)
        }
    }

    /// Set the area this [`LeafNode`] Occupies on screen.
    #[inline]
    pub fn set_rect(&mut self, new_rect: Rect) {
        self.rect = new_rect;
    }

    /// Get the length of tab list in this [`LeafNode`].
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Returns `true` when the [`LeafNode`] contains no tabs.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Get a [`Rect`] representing the area this [`LeafNode`] occupies on screen.
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Get immutable access to the ``Tab``s of this [`LeafNode`]
    #[inline]
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get mutable access to the ``Tab``s of this [`LeafNode`]
    #[inline]
    pub fn tabs_mut(&mut self) -> &mut [Tab] {
        &mut self.tabs
    }

    /// Append a ``Tab`` to the end of this [`LeafNode`]s tab list.
    ///
    /// This will also focus the added tab.
    #[track_caller]
    #[inline]
    pub fn append_tab(&mut self, tab: Tab) {
        self.active = TabIndex(self.tabs.len());
        self.tabs.push(tab);
    }

    /// Insert a ``Tab`` to this [`LeafNode`]s tab list at the specified [`TabIndex`].
    ///
    /// This will also focus the added tab.
    ///
    /// # Panics
    ///
    /// if ``tab_index`` exceeds the leaf's tab list length.
    #[track_caller]
    #[inline]
    pub fn insert_tab(&mut self, tab_index: impl Into<TabIndex>, tab: Tab) {
        let tab_index = tab_index.into();
        self.tabs.insert(tab_index.0, tab);
        self.active = tab_index;
    }

    /// Remove a ``Tab`` to this [`LeafNode`]s tab list at the specified [`TabIndex`].
    ///
    /// This will also focus the added tab.'
    ///
    /// # Panics
    ///
    /// if ``tab_index`` is out of bounds for the tab list
    #[inline]
    pub fn remove_tab(&mut self, tab_index: impl Into<TabIndex>) -> Option<Tab> {
        let index = tab_index.into();
        if index <= self.active {
            self.active.0 = self.active.0.saturating_sub(1);
        }
        Some(self.tabs.remove(index.0))
    }

    /// Removes all tabs for which `predicate` returns `false`.
    pub fn retain_tabs<F>(&mut self, predicate: F)
    where
        F: FnMut(&mut Tab) -> bool,
    {
        self.tabs.retain_mut(predicate);
    }

    /// Return the area and tab which is currently representing this [`LeafNode`]
    ///
    /// This may return ``None`` if the leaf contains 0 tabs.
    #[inline]
    pub fn active_focused(&mut self) -> Option<(Rect, &mut Tab)> {
        self.tabs
            .get_mut(self.active.0)
            .map(|tab| (self.viewport, tab))
    }
}

impl<Tab> ops::Index<TabIndex> for LeafNode<Tab> {
    type Output = Tab;

    fn index(&self, index: TabIndex) -> &Tab {
        &self.tabs[index.0]
    }
}

impl<Tab> ops::IndexMut<TabIndex> for LeafNode<Tab> {
    fn index_mut(&mut self, index: TabIndex) -> &mut Tab {
        &mut self.tabs[index.0]
    }
}
