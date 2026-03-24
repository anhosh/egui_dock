/// Dock access errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid surface that does not exist in the dock
    #[error("Invalid surface that does not exist in the dock")]
    InvalidSurface,
    /// Surface has no nodes
    #[error("Surface has no nodes")]
    EmptySurface,
    /// Invalid node that does not exist in the surface
    #[error("Invalid node that does not exist in the surface")]
    InvalidNode,
    /// The node exists but is not a leaf
    #[error("The node exists but is not a leaf")]
    NonLeafNode,
    /// Invalid tab that does not exist in the node
    #[error("Invalid tab that does not exist in the node")]
    InvalidTab,
}

/// Type alias for `Result` on [`egui_dock::Error`](Error).
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
