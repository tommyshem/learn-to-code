extern crate simple_logger;
#[macro_use]
extern crate log;

fn main() {
    simple_logger::init().unwrap();

    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
