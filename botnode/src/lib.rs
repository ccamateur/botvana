pub mod audit;
pub mod control;
pub mod engine;
pub mod error;
pub mod indicator;
pub mod market_data;
pub mod order;
pub mod trading;
pub mod util;

/// Useful prelude for implementing botnode engines
pub mod prelude {
    pub use async_codec::Framed;
    pub use async_shutdown::Shutdown;
    pub use async_trait::async_trait;
    pub use chrono::{DateTime, Utc};
    pub use futures::prelude::*;
    pub use glommio::{net::TcpStream, prelude::*, LocalExecutor, LocalExecutorBuilder};
    pub use ring_channel::*;
    pub use std::num::NonZeroUsize;
    pub use tracing::{debug, error, info, warn};

    pub use crate::engine::*;
    pub use crate::error::{EngineError, StartEngineError};
    pub use crate::indicator::IndicatorEvent;
    pub use crate::market_data::MarketEvent;
    pub use botvana::{
        cfg::{BotConfiguration, IndicatorConfig},
        net::{
            codec::BotvanaCodec,
            msg::{BotId, Message},
        },
    };

    pub type DynBoxError = Box<dyn std::error::Error>;
}
