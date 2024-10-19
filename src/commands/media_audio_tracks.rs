// Media audio track command

use std::{
    process,
    sync::{Arc, Mutex},
};

use crate::{
    api::{api_call_get_media, api_call_media_remove_audio, api_call_media_rename_audio, api_call_media_set_audio}, commands::logout::do_logout, models::{MediaAudioTrack, MediaRenameSubtitleOrAudioBody}, tools::{ensure_login, parse_identifier, parse_vault_uri}
};

use super::{
    get_vault_url, media_upload::UploaderProgressPrinter, print_request_error, CommandGlobalOptions,
};

pub async fn run_cmd_upload_media_audio_track(
    global_opts: CommandGlobalOptions,
    media: String,
    track_id: String,
    path: String,
    name: Option<String>,
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

    // Params

    let name_param = name.unwrap_or(track_id.clone());

    // Upload progress reporter

    let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

    let api_res = api_call_media_set_audio(
        &vault_url,
        media_id_param,
        track_id.clone(),
        name_param,
        path.clone(),
        global_opts.debug,
        progress_printer,
    )
    .await;

    match api_res {
        Ok(_) => {
            eprintln!("Upload completed: {path}");
            eprintln!(
                "Successfully uploaded new audio track file for #{media_id_param}: {track_id}"
            );
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

pub async fn run_cmd_rename_media_audio_track(
    global_opts: CommandGlobalOptions,
    media: String,
    track_id: String,
    new_id: Option<String>,
    new_name: Option<String>,
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

    let mut audio: Option<MediaAudioTrack> = None;

    match media_id_res {
        Ok(media_id) => {
            let media_api_res = api_call_get_media(&vault_url, media_id, global_opts.debug).await;

            match media_api_res {
                Ok(metadata) => {
                    media_id_param = media_id;

                    if let Some(audios) = metadata.audios {
                        for aud in audios {
                            if aud.id == track_id {
                                audio = Some(aud.clone());
                            }
                        }
                    } else {
                        eprintln!("Could not find audio track in media: {track_id}");
                        process::exit(1);
                    }
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

    // Call API

    let mut body = MediaRenameSubtitleOrAudioBody {
        id: track_id.clone(),
        name: "".to_string(),
    };

    if let Some(sub) = audio {
        body.name = sub.name.clone();
    } else {
        eprintln!("Could not find audio track in media: {track_id}");
        process::exit(1);
    }

    if let Some(new_id_str) = new_id {
        body.id = new_id_str;
    }

    if let Some(new_name_str) = new_name {
        body.name = new_name_str;
    }

    let final_id = body.id.clone();
    let final_name = body.name.clone();

    let api_res = api_call_media_rename_audio(
        &vault_url,
        media_id_param,
        track_id.clone(),
        &body,
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            eprintln!("Successfully renamed audio track file from #{media_id_param}: {track_id} -> {final_id} - {final_name}");
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

pub async fn run_cmd_delete_media_audio_track(
    global_opts: CommandGlobalOptions,
    media: String,
    track_id: String,
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

    // Call API

    let api_res = api_call_media_remove_audio(
        &vault_url,
        media_id_param,
        track_id.clone(),
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            eprintln!("Successfully removed audio track file from #{media_id_param}: {track_id}");
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
