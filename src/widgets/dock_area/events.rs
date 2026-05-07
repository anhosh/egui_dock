//! Events emitted by [`DockArea`](super::DockArea) during a render pass.
//!
//! The enum is intentionally coarse for now (just two variants) but
//! `#[non_exhaustive]` so future versions can split [`DockEvent::LayoutCommitted`]
//! into more specific variants (`TabClosed`, `SeparatorDragCommitted { before, after }`,
//! …) without breaking downstream consumers that go through the
//! [`DockAreaResponse::layout_changed`] / [`DockAreaResponse::layout_committed`]
//! helpers instead of pattern-matching the enum directly.

/// A single layout-affecting event observed during one render pass of a
/// [`DockArea`](super::DockArea).
///
/// Two classes are distinguished today:
///
/// * [`DockEvent::SeparatorDragging`] — fired **every frame** while the user
///   holds and drags a split separator. The dock state mutates live so the UI
///   tracks the cursor, but consumers should *not* push this to undo / persist
///   on disk on every frame.
/// * [`DockEvent::LayoutCommitted`] — fired on **finalised** mutations:
///   tab activation / close / detach / move via drag-and-drop, leaf collapse
///   toggle, window minimise toggle, surface removal, separator drag finished
///   (mouse released), separator double-click reset, separator arrow-key nudge.
///
/// The split exists so consumers can act exactly once per logical user action
/// instead of spamming `record_action` / `save_to_disk` while the user is still
/// dragging.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockEvent {
    /// Live separator drag in progress. Layout updated visually this frame;
    /// consumers should ignore this for undo / persistence and react to
    /// [`DockEvent::LayoutCommitted`] on `drag_stopped()` instead.
    SeparatorDragging,
    /// A finalised, discrete layout mutation. Use this as the trigger for
    /// undo entries and writes to disk.
    LayoutCommitted,
}

impl DockEvent {
    /// `true` for everything except [`DockEvent::SeparatorDragging`].
    ///
    /// As new variants are added in the future they must self-classify here:
    /// continuous "still happening" events return `false`, finalised actions
    /// return `true`. This keeps [`DockAreaResponse::layout_committed`] correct
    /// by construction without consumers having to update their match arms.
    #[inline]
    pub fn is_committed(&self) -> bool {
        !matches!(self, DockEvent::SeparatorDragging)
    }
}

/// Summary of what happened while rendering a [`DockArea`](super::DockArea).
///
/// Exposes the raw [`events`](Self::events) list for consumers that want to
/// inspect individual events, plus two summary helpers for the common cases:
/// "did anything change at all?" and "is there a finalised change worth
/// persisting?".
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct DockAreaResponse {
    /// Events emitted during this frame, in the order they occurred.
    pub events: Vec<DockEvent>,
}

impl DockAreaResponse {
    /// `true` if any layout mutation happened this frame, including
    /// in-progress separator drag. Layout state has changed visually but
    /// should generally not be persisted on this signal alone.
    #[inline]
    pub fn layout_changed(&self) -> bool {
        !self.events.is_empty()
    }

    /// `true` if at least one *finalised* event fired this frame
    /// (anything other than [`DockEvent::SeparatorDragging`]).
    /// This is the right trigger for undo entries and on-disk save.
    #[inline]
    pub fn layout_committed(&self) -> bool {
        self.events.iter().any(DockEvent::is_committed)
    }
}
