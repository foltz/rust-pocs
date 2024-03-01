mod tracing_writer;

use tracing::{debug, info, Level, warn};
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::FmtSubscriber;
use crate::tracing_writer::TracingWriter;

fn main() {


    // let subscriber = FmtSubscriber::builder()
    //     // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    //     // will be written to stdout.
    //     .with_max_level(Level::TRACE)
    //     // completes the builder.
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("setting default subscriber failed");


    // let f = Format::default().without_time().with_ansi(false);

    let (non_blocking, _guard) = tracing_appender::non_blocking(TracingWriter);
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_level(false)
        .with_writer(non_blocking)
        .init();

    debug!("debug message");
    warn!("warn message");
    info!("info message");

}
