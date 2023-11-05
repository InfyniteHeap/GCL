// use std::future::Future;

use gridcore::auth::microsoft_oauth2::*;

use tokio::runtime::Runtime;

static mut MICROSOFT_AUTHORIZATION_TOKEN: String = String::new();
static mut XBOX_AUTHENTICATION_TOKEN: String = String::new();
static mut XSTS_AUTHORIZATION_TOKEN: String = String::new();
static mut UHS: String = String::new();

#[test]
fn login() {
    // Create a Tokio runtime.
    let tokio_rt = Runtime::new().unwrap();
    // Suppose there is a string that holds a Microsoft authorization code.
    let auth_code = "".to_string();

    match tokio_rt.block_on(request_microsoft_oauth2_token(&auth_code)) {
        Ok(data) => match parse_response(&data) {
            Ok(data) => match fetch_data(data, "access_token") {
                // This string should be used in the next step and be taken out of this nest.
                Some(value) => unsafe { MICROSOFT_AUTHORIZATION_TOKEN = value },
                // Eject a dialog to prompt user "Failed to fetch data from response!".
                None => panic!("No contents from MS"),
            },
            // Eject a dialog to prompt user "Failed to parse response: e".
            Err(e) => panic!("{e}"),
        },
        // Eject a dialog to prompt user "Failed fetch response from remote: e".
        Err(e) => panic!("{e}"),
    }

    println!("{}", unsafe { &MICROSOFT_AUTHORIZATION_TOKEN });

    match tokio_rt.block_on(request_xbox_authentication(unsafe {
        &MICROSOFT_AUTHORIZATION_TOKEN
    })) {
        Ok(data) => match parse_response(&data) {
            Ok(data) => {
                match fetch_data(data.clone(), "Token") {
                    // This string should be used in the next step and be taken out of this nest.
                    Some(value) => unsafe { XBOX_AUTHENTICATION_TOKEN = value },
                    // Eject a dialog to prompt user "Failed to fetch data from response!".
                    None => panic!("No contents from XBOX"),
                }
            }
            // Eject a dialog to prompt user "Failed to parse response: e".
            Err(e) => panic!("{e}"),
        },
        // Eject a dialog to prompt user "Failed fetch response from remote: e".
        Err(e) => panic!("{e}"),
    }

    println!("{}", unsafe { &XBOX_AUTHENTICATION_TOKEN });

    match tokio_rt.block_on(request_xsts_authorization(unsafe {
        &XBOX_AUTHENTICATION_TOKEN
    })) {
        Ok(data) => match parse_response(&data) {
            Ok(data) => println!("{:#?}", data),
            // Eject a dialog to prompt user "Failed to parse response: e".
            Err(e) => panic!("{e}"),
        },
        // Eject a dialog to prompt user "Failed fetch response from remote: e".
        Err(e) => panic!("{e}"),
    }
}
