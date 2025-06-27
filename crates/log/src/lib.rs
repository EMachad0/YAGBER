pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) const DEFAULT_FILTER: &str = "wgpu=error,naga=warn";

pub struct LogPlugin {
    filter: String,
    level: tracing::Level,
}

impl LogPlugin {
    pub fn new() -> Self {
        Self {
            filter: DEFAULT_FILTER.to_string(),
            level: tracing::Level::INFO,
        }
    }
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Plugin for LogPlugin {
    fn init(self, _emulator: &mut yagber_app::Emulator) {
        dotenv::dotenv().ok();

        // Start with the base registry.
        let subscriber = tracing_subscriber::Registry::default();

        let default_filter = format!("{},{}", self.filter, self.level);
        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .or_else(|from_env_error| {
                _ = std::error::Error::source(&from_env_error)
                    .and_then(|source| {
                        source.downcast_ref::<tracing_subscriber::filter::ParseError>()
                    })
                    .map(|parse_err| {
                        // We cannot use the `error!` macro here because the subscriber is not ready yet.
                        eprintln!("LogPlugin failed to parse filter from env: {parse_err}");
                    });

                Ok::<tracing_subscriber::EnvFilter, tracing_subscriber::filter::FromEnvError>(
                    tracing_subscriber::EnvFilter::builder().parse_lossy(&default_filter),
                )
            })
            .unwrap();

        let subscriber = subscriber.with(filter_layer);

        let stderr_fmt_layer =
            tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr);
        let subscriber = subscriber.with(stderr_fmt_layer);

        #[cfg(feature = "tracing-chrome")]
        let subscriber = {
            let mut chrome_layer_builder = tracing_chrome::ChromeLayerBuilder::new();
            if let Ok(path) = std::env::var("TRACE_CHROME") {
                chrome_layer_builder = chrome_layer_builder.file(path);
            } else {
                chrome_layer_builder = chrome_layer_builder.file("out/yagber.trace.json");
            }

            let (chrome_layer, guard) = chrome_layer_builder.build();
            let guard = ChromeLayerGuard { _guard: guard };
            _emulator.with_component(guard);
            println!("Chrome layer added");
            subscriber.with(chrome_layer)
        };

        let _ = subscriber.try_init();
        info!("Yagber tracing initialized");
    }
}

#[cfg(feature = "tracing-chrome")]
pub struct ChromeLayerGuard {
    _guard: tracing_chrome::FlushGuard,
}

#[cfg(feature = "tracing-chrome")]
impl yagber_app::Component for ChromeLayerGuard {}

pub fn init_tracing() {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .event_format(tracing_subscriber::fmt::format().compact())
        .try_init();
    info!("Yagber tracing initialized");
}
