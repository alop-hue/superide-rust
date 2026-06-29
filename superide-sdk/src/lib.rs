pub mod agent;
pub mod event;
pub mod extension;
pub mod provider;

use std::{future::Future, pin::Pin};

pub type SdkFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
