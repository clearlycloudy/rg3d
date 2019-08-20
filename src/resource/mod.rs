pub mod texture;
pub mod fbx;
pub mod model;
pub mod ttf;

use serde::{Serialize, Deserialize};
use std::path::*;
use crate::{
    resource::{
        texture::*,
        model::{Model}
    }
};
use std::any::{TypeId, Any};

#[derive(Serialize, Deserialize)]
pub enum ResourceKind {
    Texture(Texture),
    Model(Model),
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    path: PathBuf,
    kind: ResourceKind
}

impl Resource {
    pub fn new(path: &Path, kind: ResourceKind) -> Resource {
        Resource {
            path: path.to_path_buf(),
            kind,
        }
    }

    #[inline]
    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    #[inline]
    pub fn borrow_kind(&self) -> &ResourceKind {
        &self.kind
    }

    #[inline]
    pub fn borrow_kind_mut(&mut self) -> &mut ResourceKind {
        &mut self.kind
    }

    pub fn get_kind_id(&self) -> TypeId {
        match &self.kind {
            ResourceKind::Model(model) => model.type_id(),
            ResourceKind::Texture(texture) => texture.type_id()
        }
    }
}

pub trait ResourceBehavior {
    fn load(&self);
}