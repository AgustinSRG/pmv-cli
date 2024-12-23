// Album export

use std::process;

use crate::{
    api::api_call_get_album,
    commands::{
        logout::do_logout,
        media_export::{download_media_asset, run_cmd_export_media},
    },
    models::{Album, AlbumMetadataExport},
    tools::{ask_user, ensure_login, get_extension_from_url, parse_identifier, parse_vault_uri},
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_export_album(
    global_opts: CommandGlobalOptions,
    album: String,
    output: Option<String>,
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

    // Params

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

    // Get album metadata

    let api_get_album_res = api_call_get_album(&vault_url, album_id, global_opts.debug).await;

    let album_metadata: Album = match api_get_album_res {
        Ok(meta) => meta,
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
    };

    // Output folder

    let out_folder: String = match output {
        Some(o) => o,
        None => {
            format!("album_{album_id}")
        }
    };

    let out_exists = std::path::Path::new(&out_folder).exists();

    if out_exists && !global_opts.auto_confirm {
        eprintln!("The folder {out_folder} already exists");
        let confirmation = ask_user("Do you want to overwrite it? y/n: ")
            .await
            .unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    if !out_exists {
        let create_dir_res = tokio::fs::create_dir(std::path::Path::new(&out_folder)).await;

        match create_dir_res {
            Ok(_) => {}
            Err(e) => {
                let e_str = e.to_string();
                eprintln!("Could not create the folder {out_folder}. Error: {e_str}");
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

    // Metadata

    let mut out_metadata = AlbumMetadataExport {
        name: album_metadata.name,
        media_list: album_metadata
            .list
            .iter()
            .map(|m| ("media_".to_string() + &m.id.to_string()).to_string())
            .collect(),
        thumbnail: None,
    };

    // Thumbnail

    if let Some(album_thumbnail) = album_metadata.thumbnail {
        if !album_thumbnail.is_empty() {
            let ext = get_extension_from_url(&album_thumbnail, "jpg");
            let out_file_name = "thumbnail".to_owned() + "." + &ext;

            let thumbnail_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                "thumbnail",
                album_thumbnail,
                thumbnail_out_path,
                logout_after_operation,
            )
            .await;

            out_metadata.thumbnail = Some(out_file_name);
        }
    }

    // Write metadata

    let metadata_file = "metadata.json".to_string();
    let metadata_out_path = std::path::Path::new(&out_folder)
        .join(&metadata_file)
        .to_str()
        .unwrap()
        .to_string();

    let metadata_str = serde_json::to_string(&out_metadata).unwrap();

    let meta_write_res = tokio::fs::write(metadata_out_path.clone(), metadata_str).await;

    match meta_write_res {
        Ok(_) => {}
        Err(e) => {
            let e_str = e.to_string();
            eprintln!("Could not write metadata file: {metadata_out_path}. Error: {e_str}");
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

    // Export media

    for media_item in album_metadata.list {
        let media_folder = ("media_".to_string() + &media_item.id.to_string()).to_string();

        let media_out_path = std::path::Path::new(&out_folder)
            .join(&media_folder)
            .to_str()
            .unwrap()
            .to_string();

        run_cmd_export_media(
            global_opts.clone(),
            media_item.id.to_string(),
            Some(media_out_path),
            true,
        )
        .await;
    }

    // Done

    if logout_after_operation {
        let logout_res = do_logout(&global_opts, &vault_url).await;

        match logout_res {
            Ok(_) => {}
            Err(_) => {
                process::exit(1);
            }
        }
    }
    eprintln!("Done. Successfully exported album into folder {out_folder}");
}
