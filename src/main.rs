use dittolive_ditto::{identity::*, prelude::*};
use std::sync::mpsc::channel;
use std::{self, str::FromStr, sync::Arc};
use structopt::StructOpt;
use serde_json::json;

#[derive(StructOpt)]
struct Args {
    #[structopt(long, env = "APP_ID")]
    app_id: String,
    #[structopt(long, env = "SHARED_TOKEN")]
    shared_token: String,
    #[structopt(long, env = "COLLECTION")]
    collection: String,
    #[structopt(long, env = "MAC_ADDRESS")]
    mac_address: String,
    #[structopt(long, env = "IP_ADDRESS")]
    ip_address: String,
}

fn main() -> Result<(), DittoError> {
    let args = Args::from_args();
    let (sender, receiver) = channel::<(Vec<BoxedDocument>, LiveQueryEvent)>();
    let event_handler = move |documents: Vec<BoxedDocument>, event: LiveQueryEvent| {
        sender.send((documents, event)).unwrap();
    };

    let ditto = Ditto::builder()
        .with_root(Arc::new(PersistentRoot::from_current_exe()?))
        .with_minimum_log_level(LogLevel::Verbose)
        .with_identity(|ditto_root| {
            let app_id = AppId::from_str(&args.app_id).unwrap();
            let shared_token = args.shared_token;
            let enable_cloud_sync = true;
            let custom_auth_url = None;
            OnlinePlayground::new(
                ditto_root,
                app_id,
                shared_token,
                enable_cloud_sync,
                custom_auth_url,
            )
        })?
        .build()?;

    ditto.start_sync().unwrap();
    let store = ditto.store();

    let mac_address = args.mac_address;
    let ip_address = args.ip_address;
    let collection = args.collection;

    let device_info = json!({
        "mac_address": mac_address,
        "ip_address": ip_address,
    });
    let collection_id = store.collection(&collection).unwrap();
    let id = collection_id.upsert(device_info).unwrap();

    println!("*** Inserted document with id={}", id);

    Ok(()) //without ; so that this value is returned; and provide empty argument (), because
    //when result is OK, the no value should be returned

}
