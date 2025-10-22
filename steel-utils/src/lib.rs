#![feature(
    const_trait_impl,
    const_slice_make_iter,
    const_cmp,
    derive_const,
    core_intrinsics
)]
#![allow(internal_features)]
use std::{borrow::Cow, str::FromStr};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::math::{vector2::Vector2, vector3::Vector3};

pub mod math;

// A wrapper that allows us to do debug checks on the lock. To prevent deadlocks.
#[derive(Debug)]
pub struct SteelRwLock<T> {
    inner: RwLock<T>,
}

impl<T> SteelRwLock<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: RwLock::new(inner),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().await
    }
}

// Wrapper types making it harder to accidentaly use the wrong underlying type.

// A raw block state id. Using the registry this id can be derived into a block and it's current properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStateId(pub u16);

// A chunk position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos(pub Vector2<i32>);

// A block position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos(pub Vector3<i32>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceLocation {
    pub namespace: Cow<'static, str>,
    pub path: Cow<'static, str>,
}

impl ResourceLocation {
    pub const VANILLA_NAMESPACE: &'static str = "minecraft";

    pub fn vanilla(path: String) -> Self {
        ResourceLocation {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Owned(path),
        }
    }

    pub const fn vanilla_static(path: &'static str) -> Self {
        ResourceLocation {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Borrowed(path),
        }
    }

    pub fn valid_namespace_char(namespace_char: char) -> bool {
        namespace_char == '_'
            || namespace_char == '-'
            || namespace_char.is_ascii_lowercase()
            || namespace_char.is_ascii_digit()
            || namespace_char == '.'
    }

    pub fn valid_path_char(path_char: char) -> bool {
        path_char == '_'
            || path_char == '-'
            || path_char.is_ascii_lowercase()
            || path_char.is_ascii_digit()
            || path_char == '/'
            || path_char == '.'
    }

    pub fn validate_namespace(namespace: &str) -> bool {
        namespace.chars().all(Self::valid_namespace_char)
    }

    pub fn validate_path(path: &str) -> bool {
        path.chars().all(Self::valid_path_char)
    }

    pub fn validate(namespace: &str, path: &str) -> bool {
        Self::validate_namespace(namespace) && Self::validate_path(path)
    }
}

impl ToString for ResourceLocation {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

impl FromStr for ResourceLocation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid resource location: {}", s));
        }

        if !ResourceLocation::validate_namespace(parts[0]) {
            return Err(format!("Invalid namespace: {}", parts[0]));
        }

        if !ResourceLocation::validate_path(parts[1]) {
            return Err(format!("Invalid path: {}", parts[1]));
        }

        Ok(ResourceLocation {
            namespace: Cow::Owned(parts[0].to_string()),
            path: Cow::Owned(parts[1].to_string()),
        })
    }
}
