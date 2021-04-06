// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A crate that contains foundational building blocks for the IOTA Tangle.

// #![warn(missing_docs)]

pub mod config;
pub mod event;
pub mod flags;
pub mod metadata;
pub mod ms_tangle;
pub mod pruning;
pub mod storage;
pub mod traversal;
pub mod unconfirmed_message;
pub mod urts;
pub mod vec_set;
pub mod worker;

mod tangle;
mod vertex;

pub use ms_tangle::MsTangle;
pub use tangle::{Hooks, Tangle};
pub use urts::BELOW_MAX_DEPTH;
pub use worker::TangleWorker;

use crate::vec_set::VecSet;

use bee_message::Message;
use bee_runtime::node::{Node, NodeBuilder};
use bee_snapshot::config::SnapshotConfig;

use std::{ops::Deref, sync::Arc};

/// A thread-safe reference to a `Message`.
#[derive(Clone)]
pub struct MessageRef(pub(crate) Arc<Message>);

impl Deref for MessageRef {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub fn init<N: Node>(
    snapshot_config: &SnapshotConfig,
    pruning_config: &pruning::PruningConfig,
    node_builder: N::Builder,
) -> N::Builder
where
    N::Backend: storage::StorageBackend,
{
    node_builder
        .with_worker::<TangleWorker>()
        .with_worker_cfg::<pruning::PrunerWorker>((snapshot_config.clone(), pruning_config.clone()))
}
