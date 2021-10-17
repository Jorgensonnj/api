use tracing_subscriber::{fmt::{Layer, MakeWriter}, layer::SubscriberExt, EnvFilter, Registry};
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;

pub fn get_subscriber(
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    // Get the logging level from the env if not specified
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Create a layer as to where the log is going to be written
    let print_layer = Layer::new()
        .with_writer(sink);

   // Creates a subscriber using the filter and layer
    Registry::default()
        .with(filter_layer)
        .with(print_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

