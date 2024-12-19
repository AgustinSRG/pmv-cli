// Album thumbnail update command

use std::{
    process,
    sync::{Arc, Mutex},
};

use crate::{
    api::api_call_album_change_thumbnail,
    commands::logout::do_logout,
    tools::{ensure_login, parse_identifier, parse_vault_uri},
};

use super::{
    get_vault_url, media_upload::UploaderProgressPrinter, print_request_error, CommandGlobalOptions,
};

pub async fn run_cmd_upload_album_thumbnail(
    global_opts: CommandGlobalOptions,
    album: String,
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

    // Album ID

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
            eprintln!("Invalid album identifier specified.");
            process::exit(1);
        }
    };

    // Upload progress reporter

    let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

    let api_res = api_call_album_change_thumbnail(
        &vault_url,
        album_id,
        path.clone(),
        global_opts.debug,
        progress_printer,
    )
    .await;

    match api_res {
        Ok(upload_res) => {
            eprintln!("Upload completed: {path}");

            let thumb_new_url = upload_res.url;

            eprintln!("Successfully updated the thumbnail of album #{album_id}: {thumb_new_url}");

            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            print_request_error(e);
            process::exit(1);
        }
    }
}
