// Time slices commands

use std::process;

use crate::{
    api::{api_call_get_media, api_call_media_change_time_slices},
    commands::logout::do_logout,
    models::MediaTimeSlice,
    tools::{ensure_login, parse_identifier, parse_vault_uri},
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_get_media_time_slices(global_opts: CommandGlobalOptions, media: String) -> () {
    let url_parse_res = parse_vault_uri(get_vault_url(global_opts.vault_url.clone()));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(vault_url, None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Params

    let media_id_res = parse_identifier(&media);
    let media_id: u64;

    match media_id_res {
        Ok(id) => {
            media_id = id;
        }
        Err(_) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Invalid media identifier specified.");
            process::exit(1);
        }
    }

    // Call API

    let api_res = api_call_get_media(vault_url.clone(), media_id, global_opts.debug).await;

    match api_res {
        Ok(media_data) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            match media_data.time_slices {
                Some(time_slices) => {
                    if time_slices.is_empty() {
                        eprintln!("The media asset #{media_id} does not have time slices");
                    } else {
                        let time_slices_str = MediaTimeSlice::vector_to_string(&time_slices);
                        println!("{time_slices_str}");
                    }
                }
                None => {
                    eprintln!("The media asset #{media_id} does not have time slices");
                }
            }
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_set_media_time_slices(
    global_opts: CommandGlobalOptions,
    media: String,
    path: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(global_opts.vault_url.clone()));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(vault_url, None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Media ID

    let media_id_res = parse_identifier(&media);

    let media_id_param: u64;

    match media_id_res {
        Ok(media_id) => {
            let media_api_res =
                api_call_get_media(vault_url.clone(), media_id, global_opts.debug).await;

            match media_api_res {
                Ok(_) => {
                    media_id_param = media_id;
                }
                Err(e) => {
                    print_request_error(e);

                    if logout_after_operation {
                        let logout_res = do_logout(global_opts, vault_url.clone()).await;

                        match logout_res {
                            Ok(_) => {}
                            Err(_) => {
                                process::exit(1);
                            }
                        }
                    }
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Invalid media asset identifier specified.");
            process::exit(1);
        }
    }

    let time_slices_file_read_res = tokio::fs::read_to_string(&path).await;

    let time_slices: Vec<MediaTimeSlice>;

    match time_slices_file_read_res {
        Ok(time_slices_str) => {
            let time_slices_res = MediaTimeSlice::parse_vector(&time_slices_str);

            match time_slices_res {
                Ok(time_slices_v) => {
                    time_slices = time_slices_v;
                }
                Err(_) => {
                    eprintln!("Error: The file {path} does not contain a valid set of time slices");
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            let e_str = e.to_string();
            eprintln!("Error reading the file {path}: {e_str}");
            process::exit(1);
        }
    }

    // Call API

    let api_res = api_call_media_change_time_slices(
        vault_url.clone(),
        media_id_param,
        time_slices,
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            eprintln!("Successfully updated the time_slices of #{media_id_param}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}
