// Media export command

use std::{
    process,
    sync::{Arc, Mutex},
};

use crate::{
    api::{api_call_album_change_thumbnail, api_call_create_album},
    commands::{logout::do_logout, media_import::run_cmd_import_media},
    models::{AlbumMetadataExport, AlbumNameBody},
    tools::{ensure_login, parse_vault_uri},
};

use super::{
    get_vault_url, media_upload::UploaderProgressPrinter, print_request_error, CommandGlobalOptions,
};

pub async fn run_cmd_import_album(global_opts: CommandGlobalOptions, path: String) {
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

    // Metadata file

    let metadata_file = std::path::Path::new(&path)
        .join("metadata.json")
        .to_str()
        .unwrap()
        .to_string();

    let import_metadata: AlbumMetadataExport;
    let metadata_file_res = tokio::fs::read_to_string(&metadata_file).await;

    match metadata_file_res {
        Ok(metadata_str) => {
            let parsed_metadata: Result<AlbumMetadataExport, _> =
                serde_json::from_str(&metadata_str);

            match parsed_metadata {
                Ok(m) => {
                    import_metadata = m;
                }
                Err(e) => {
                    let e_str = e.to_string();
                    eprintln!("Could not read metadata file. Error: {e_str}");
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
        Err(e) => {
            let e_str = e.to_string();
            eprintln!("Could not read metadata file. Error: {e_str}");
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

    // Create album

    let album_id: u64;
    let album_name = import_metadata.name;

    let api_res = api_call_create_album(
        &vault_url,
        AlbumNameBody {
            name: album_name.clone(),
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(added_album) => {
            album_id = added_album.album_id;

            eprintln!("Successfully created album #{album_id}: {album_name}");
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

    // Set thumbnail

    if let Some(thumbnail_file) = import_metadata.thumbnail {
        let thumbnail_file_path = std::path::Path::new(&path)
            .join(&thumbnail_file)
            .to_str()
            .unwrap()
            .to_string();

        let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

        let api_res = api_call_album_change_thumbnail(
            &vault_url,
            album_id,
            thumbnail_file_path.clone(),
            global_opts.debug,
            progress_printer,
        )
        .await;

        match api_res {
            Ok(upload_res) => {
                eprintln!("Upload completed: {thumbnail_file_path}");

                let thumb_new_url = upload_res.url;

                eprintln!(
                    "Successfully updated the thumbnail of album #{album_id}: {thumb_new_url}"
                );
            }
            Err(e) => {
                print_request_error(e);
            }
        }
    }

    // Import media

    for media_folder in import_metadata.media_list {
        let media_path = std::path::Path::new(&path)
            .join(&media_folder)
            .to_str()
            .unwrap()
            .to_string();

        run_cmd_import_media(
            global_opts.clone(),
            media_path,
            Some(album_id.to_string()),
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

    eprintln!("Done. Successfully imported album #{album_id} - {album_name}");
}
