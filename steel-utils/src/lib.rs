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

pub mod locks;
pub mod math;
pub mod text;
pub mod types;
