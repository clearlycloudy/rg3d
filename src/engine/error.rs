//! All possible errors that can happen in the engine.

use crate::{renderer::error::RendererError, sound::error::SoundError};
use glutin::{ContextError, CreationError};

/// See module docs.
#[derive(Debug)]
pub enum EngineError {
    /// Sound system error.
    Sound(SoundError),
    /// Rendering system error.
    Renderer(RendererError),
    /// OpenGL context creation error.
    ContextCreationError(CreationError),
    /// Runtime OpenGL context error.
    ContextError(ContextError),
}

impl From<SoundError> for EngineError {
    fn from(sound: SoundError) -> Self {
        EngineError::Sound(sound)
    }
}

impl From<RendererError> for EngineError {
    fn from(renderer: RendererError) -> Self {
        EngineError::Renderer(renderer)
    }
}

impl From<CreationError> for EngineError {
    fn from(e: CreationError) -> Self {
        EngineError::ContextCreationError(e)
    }
}

impl From<ContextError> for EngineError {
    fn from(e: ContextError) -> Self {
        EngineError::ContextError(e)
    }
}
