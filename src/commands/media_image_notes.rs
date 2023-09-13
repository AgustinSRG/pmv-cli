// Image notes commands

use std::process;

use crate::{
    api::{api_call_get_media, api_call_media_change_notes},
    commands::logout::do_logout,
    models::ImageNote,
    tools::{ensure_login, parse_identifier, parse_vault_uri},
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_set_media_image_notes(
    global_opts: CommandGlobalOptions,
    media: String,
    path: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

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
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

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
                api_call_get_media(&vault_url, media_id, global_opts.debug).await;

            match media_api_res {
                Ok(_) => {
                    media_id_param = media_id;
                }
                Err(e) => {
                    print_request_error(e);

                    if logout_after_operation {
                        let logout_res = do_logout(&global_opts, &vault_url).await;

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
                let logout_res = do_logout(&global_opts, &vault_url).await;

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

    let image_notes_read_res = tokio::fs::read_to_string(&path).await;

    let image_notes: Vec<ImageNote>;

    match image_notes_read_res {
        Ok(image_notes_str) => {
            let parsed_notes_res: Result<Vec<ImageNote>, _> =
                serde_json::from_str(&image_notes_str);

            match parsed_notes_res {
                Ok(parsed_notes) => {
                    image_notes = parsed_notes;
                }
                Err(_) => {
                    eprintln!("Error: The file {path} does not contain a valid set of image notes");
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

    let api_res = api_call_media_change_notes(
        &vault_url,
        media_id_param,
        image_notes,
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            eprintln!("Successfully updated the image notes of #{media_id_param}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

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
