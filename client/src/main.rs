use anyhow::Result;
use notify_rust::Notification;
use rodio::{source::Source, Decoder, OutputStream};
use rust_socketio::{ClientBuilder, Payload, RawClient};
use std::fs::File;
use std::io::BufReader;
use std::{env, thread, time::Duration};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

fn main() -> Result<()> {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()?;
    let backend_url: String = env::var("BACKEND_URL")?;

    let socket = ClientBuilder::new(&backend_url)
        .namespace("/")
        .on("trigger-alert", trigger_alert)
        .on("error", |err, _| error!("{:?}", err))
        .connect()?;

    // FIXME: This works, but there is probably a better way to do this.
    loop {
        thread::sleep(Duration::from_secs(5)); // Sleep for 60 seconds, then repeat
    }

    // socket.disconnect()?;
    #[allow(unreachable_code)]
    Ok(())
}

fn trigger_alert(payload: Payload, socket: RawClient) {
    info!("Triggering alert!");

    let nf_summary = env::var("NOTIFICATION_SUMMARY").unwrap();
    let nf_body = env::var("NOTIFICATION_BODY").unwrap();
    let nf_timeout = env::var("NOTIFICATION_TIMEOUT").unwrap();
    let nf_timeout = nf_timeout.parse::<u32>().unwrap();

    Notification::new()
        .summary(&nf_summary)
        .body(&nf_body)
        .timeout(notify_rust::Timeout::Milliseconds(nf_timeout)) //milliseconds
        .show()
        .unwrap();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("./sound.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
