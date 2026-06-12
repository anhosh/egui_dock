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

    /// The tab that was active immediately *before* [`active`](Self::active),
    /// or `None` if there is no such history (fresh leaf, or the previous
    /// active tab has since been removed).
    ///
    /// # Why this exists
    ///
    /// `active` is a *positional* index, not a tab identity. When the active
    /// tab is removed (closed / moved out via a split / detached) we must pick
    /// a new active tab. The naive choice — the left neighbour (`active - 1`) —
    /// is surprising: open a non-last tab, append a new one (which auto-focuses),
    /// then move that new tab elsewhere, and the leaf jumps to the neighbour
    /// instead of returning to the tab you were actually looking at.
    /// `prev_active` records "where we came from" so removal of the active tab
    /// falls back to it instead.
    ///
    /// # Invariant (upheld by every method on this type)
    ///
    /// * Either `None`, or `Some(i)` with `i < tabs.len()` and `i != active`.
    /// * Updated **only** through the methods below — never assign `active`
    ///   directly from outside. UI activation paths must go through
    ///   [`set_active_tab`](Self::set_active_tab) or
    ///   [`activate_tab_remembering`](Self::activate_tab_remembering) so this
    ///   field stays in sync. One place per mutation touches it — that is what
    ///   keeps this addition easy to maintain.
    #[cfg_attr(feature = "serde", serde(default))]
    pub prev_active: Option<TabIndex>,

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
            prev_active: None,
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
            self.activate_tab_remembering(index);
            Ok(())
        } else {
            Err(Error::InvalidTab)
        }
    }

    /// Make `index` the active tab, recording the previously-active tab in
    /// [`prev_active`](Self::prev_active) so a later removal of `index` can fall
    /// back to it. A no-op (and history-preserving) if `index` is already active.
    ///
    /// `index` is assumed to be in bounds — this is the single chokepoint every
    /// "switch to this tab" path (including the UI tab-bar click handlers) must
    /// funnel through, so the [`prev_active`](Self::prev_active) invariant cannot
    /// drift out of sync.
    #[inline]
    pub fn activate_tab_remembering(&mut self, index: TabIndex) {
        if index != self.active {
            self.prev_active = Some(self.active);
            self.active = index;
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
        // Appending happens at the end, so existing indices (including the old
        // active) keep their positions — record the old active verbatim as the
        // tab to return to if this freshly-focused tab is later removed.
        self.prev_active = (!self.tabs.is_empty()).then_some(self.active);
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
        // Capture the old active before the insertion shifts indices: any tab
        // at or after the insertion point moves one slot to the right.
        self.prev_active = (!self.tabs.is_empty()).then(|| {
            let old = self.active.0;
            TabIndex(if old >= tab_index.0 { old + 1 } else { old })
        });
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
        if index == self.active {
            // Removing the active tab: fall back to the previously-active tab
            // if we still have a valid record of it (the common case behind the
            // "split moved the wrong tab into focus" bug). Otherwise keep the
            // old left-neighbour behaviour.
            match self.prev_active {
                // `prev != index` always holds by the invariant, but guard anyway.
                Some(prev) if prev != index => {
                    // `prev` survives the removal; shift it left if it sat to the
                    // right of the tab being removed.
                    self.active = if prev.0 > index.0 {
                        TabIndex(prev.0 - 1)
                    } else {
                        prev
                    };
                }
                _ => self.active.0 = self.active.0.saturating_sub(1),
            }
            // The history slot is consumed: the tab we came from is now active,
            // so there is no longer a meaningful "before that" to keep.
            self.prev_active = None;
        } else {
            // Removing some other tab: keep `active` pointing at the same tab,
            // shifting its index if the removal was to its left.
            if index.0 < self.active.0 {
                self.active.0 -= 1;
            }
            // Keep `prev_active` pointing at the same tab as well.
            self.prev_active = match self.prev_active {
                Some(prev) if prev == index => None, // the remembered tab is gone
                Some(prev) if index.0 < prev.0 => Some(TabIndex(prev.0 - 1)),
                other => other,
            };
        }
        Some(self.tabs.remove(index.0))
    }

    /// Removes all tabs for which `predicate` returns `false`.
    pub fn retain_tabs<F>(&mut self, predicate: F)
    where
        F: FnMut(&mut Tab) -> bool,
    {
        self.tabs.retain_mut(predicate);
        // A bulk retain can drop arbitrary tabs, so the recorded "previous
        // active" index can no longer be trusted — drop it.
        self.prev_active = None;
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

#[cfg(test)]
mod prev_active_tests {
    use super::LeafNode;
    use crate::{TabIndex, Tree};

    /// Build a leaf with the given tabs and active index set *via the public
    /// path* (`set_active_tab`) so `prev_active` is initialised the same way the
    /// real UI would set it.
    fn leaf_active(tabs: &[char], active: usize) -> LeafNode<char> {
        let mut leaf = LeafNode::new(tabs.to_vec());
        leaf.set_active_tab(TabIndex(active)).unwrap();
        leaf
    }

    /// The exact reported bug: open a non-last tab, append a new tab (which
    /// auto-focuses), then remove that appended tab (as a split/move does).
    /// The leaf must return to the tab that was active *before* the append,
    /// not the appended tab's left neighbour.
    #[test]
    fn append_then_remove_active_returns_to_prior_tab() {
        // tabs [A,B,C,D], active = C (index 2)
        let mut leaf = leaf_active(&['A', 'B', 'C', 'D'], 2);
        assert_eq!(leaf.prev_active, Some(TabIndex(0))); // set_active recorded default-0

        // append E -> auto-focus, prev becomes C(2)
        leaf.append_tab('E');
        assert_eq!(leaf.active, TabIndex(4));
        assert_eq!(leaf.prev_active, Some(TabIndex(2)));

        // move E out == remove the active tab
        let removed = leaf.remove_tab(TabIndex(4));
        assert_eq!(removed, Some('E'));
        assert_eq!(leaf.tabs, vec!['A', 'B', 'C', 'D']);
        // BEFORE the fix this was D (index 3); now it is C (index 2).
        assert_eq!(leaf.active, TabIndex(2));
        assert_eq!(leaf.prev_active, None, "history slot consumed on fallback");
    }

    #[test]
    fn append_on_fresh_leaf_has_no_history() {
        let mut leaf = LeafNode::new(Vec::<char>::new());
        leaf.append_tab('A');
        assert_eq!(leaf.active, TabIndex(0));
        assert_eq!(leaf.prev_active, None);
    }

    #[test]
    fn remove_active_without_history_uses_left_neighbour() {
        // No set_active call -> prev_active is None.
        let mut leaf = LeafNode::new(vec!['A', 'B', 'C']);
        leaf.set_active_tab(TabIndex(2)).unwrap(); // prev = 0
        // First removal of active consumes the history...
        leaf.remove_tab(TabIndex(2)); // active -> prev (0)
        assert_eq!(leaf.active, TabIndex(0));
        assert_eq!(leaf.prev_active, None);
        // ...so a second removal of the active tab has no history and falls
        // back to the classic left-neighbour rule (saturating at 0).
        leaf.remove_tab(TabIndex(0));
        assert_eq!(leaf.tabs, vec!['B']);
        assert_eq!(leaf.active, TabIndex(0));
    }

    #[test]
    fn insert_tab_shifts_recorded_prev_active() {
        // tabs [A,B,C], active C(2), prev recorded as default 0.
        let mut leaf = leaf_active(&['A', 'B', 'C'], 2);
        // Insert X at front and focus it: everything shifts right by one.
        leaf.insert_tab(TabIndex(0), 'X');
        assert_eq!(leaf.tabs, vec!['X', 'A', 'B', 'C']);
        assert_eq!(leaf.active, TabIndex(0));
        assert_eq!(leaf.prev_active, Some(TabIndex(3))); // old C, shifted 2 -> 3

        // Remove the focused X -> fall back to the shifted prior tab C.
        leaf.remove_tab(TabIndex(0));
        assert_eq!(leaf.tabs, vec!['A', 'B', 'C']);
        assert_eq!(leaf.active, TabIndex(2)); // C again
    }

    #[test]
    fn removing_remembered_tab_clears_history() {
        // active C(2), prev = A(0).
        let mut leaf = leaf_active(&['A', 'B', 'C', 'D'], 2);
        assert_eq!(leaf.prev_active, Some(TabIndex(0)));
        // Remove A (the remembered tab, not the active one).
        leaf.remove_tab(TabIndex(0));
        assert_eq!(leaf.tabs, vec!['B', 'C', 'D']);
        assert_eq!(leaf.active, TabIndex(1)); // C shifted left
        assert_eq!(leaf.prev_active, None, "remembered tab gone");
    }

    #[test]
    fn removing_tab_right_of_active_leaves_indices_untouched() {
        // active B(1), prev = A(0).
        let mut leaf = leaf_active(&['A', 'B', 'C', 'D'], 1);
        leaf.remove_tab(TabIndex(3)); // remove D, to the right of both
        assert_eq!(leaf.tabs, vec!['A', 'B', 'C']);
        assert_eq!(leaf.active, TabIndex(1));
        assert_eq!(leaf.prev_active, Some(TabIndex(0)));
    }

    #[test]
    fn prev_active_tracks_only_one_level() {
        let mut leaf = LeafNode::new(vec!['A', 'B', 'C']);
        leaf.set_active_tab(TabIndex(1)).unwrap(); // active B, prev A(0)
        assert_eq!(leaf.prev_active, Some(TabIndex(0)));
        leaf.set_active_tab(TabIndex(2)).unwrap(); // active C, prev B(1)
        assert_eq!(leaf.prev_active, Some(TabIndex(1)));
        // Removing active C returns to B, not A.
        leaf.remove_tab(TabIndex(2));
        assert_eq!(leaf.active, TabIndex(1));
    }

    #[test]
    fn reactivating_same_tab_is_a_noop_for_history() {
        let mut leaf = leaf_active(&['A', 'B', 'C'], 2); // prev = 0
        leaf.set_active_tab(TabIndex(2)).unwrap(); // same active -> no change
        assert_eq!(leaf.prev_active, Some(TabIndex(0)));
    }

    #[test]
    fn retain_drops_history() {
        let mut leaf = leaf_active(&['A', 'B', 'C'], 2); // prev = 0
        leaf.retain_tabs(|t| *t != 'B');
        assert_eq!(leaf.prev_active, None);
    }

    /// `Tree::push_to_first_leaf` auto-focuses the pushed tab, so it is an
    /// active-changing site and must funnel through `append_tab` like every
    /// other one — otherwise the bug above survives on that path.
    #[test]
    fn push_to_first_leaf_records_history() {
        let mut tree = Tree::new(vec!['A', 'B', 'C']);
        let root = crate::NodeIndex::root();
        tree[root]
            .get_leaf_mut()
            .unwrap()
            .set_active_tab(TabIndex(1))
            .unwrap();

        tree.push_to_first_leaf('D');

        let leaf = tree[root].get_leaf_mut().unwrap();
        assert_eq!(leaf.active, TabIndex(3), "the pushed tab is focused");
        assert_eq!(leaf.prev_active, Some(TabIndex(1)));

        // Moving the pushed tab out returns to B, not to its left neighbour C.
        leaf.remove_tab(TabIndex(3));
        assert_eq!(leaf.active, TabIndex(1));
    }
}
