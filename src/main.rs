use dittolive_ditto::{prelude::*};
//use dittolive_ditto::{identity, prelude::*};
use std::sync::Arc;

fn main() -> Result<(), DittoError> {

    println!("Hello, ditto!");

    let ditto = Ditto::builder() //@@@why no ; after this line?
     // creates a `ditto_data` folder in the directory containing the executing process
    .with_root(Arc::new(PersistentRoot::from_current_exe()?))
    .with_identity(|ditto_root| {
        // Provided as an env var, may also be provided as hardcoded string
        //let app_id  = "0219fe97-ccff-4b24-99d2-140f54ba9a60";
        let shared_token = "0aa8abeb-c950-4d5d-91da-e0a8fc9aa90a".to_string();
        //let str1 = "0219fe97-ccff-4b24-99d2-140f54ba9a60";
        //let app_id = AppId::as_str(str1 as AppId);
        //let app_id = AppId::from_env("0219fe97-ccff-4b24-99d2-140f54ba9a60")?;
        let app_id = AppId::generate(); //@@@reading from env give run time error, NotPresent

        //let shared_token = std::env::var("0aa8abeb-c950-4d5d-91da-e0a8fc9aa90a").unwrap();
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

    ditto.start_sync()?;
    println!("ditto sync started\n");
    Ok(()) //without ; so that this value is returned; and provide empty argument (), because
        //when result is OK, the no value should be returned

}
