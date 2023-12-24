// Media import command

use std::{
    process,
    sync::{Arc, Mutex},
};

use crate::{
    api::{
        api_call_get_media, api_call_media_add_attachment, api_call_media_change_description,
        api_call_media_change_extended_description, api_call_media_change_extra,
        api_call_media_change_notes, api_call_media_change_thumbnail,
        api_call_media_change_time_slices, api_call_media_rename_attachment,
        api_call_media_set_audio, api_call_media_set_subtitle, api_call_tag_add,
        api_call_upload_media,
    },
    commands::logout::do_logout,
    models::{
        AddTagBody, ImageNote, MediaMetadataExport, MediaRenameAttachmentBody,
        MediaUpdateDescriptionBody, MediaUpdateExtendedDescriptionBody, MediaUpdateExtraBody,
    },
    tools::{
        ensure_login, identifier_to_string, parse_identifier, parse_vault_uri, to_csv_string,
        ProgressReceiver,
    },
};

use super::{
    get_vault_url,
    media_upload::{EncryptionProgressPrinter, UploaderProgressPrinter},
    print_request_error, CommandGlobalOptions,
};

pub async fn run_cmd_import_media(
    global_opts: CommandGlobalOptions,
    path: String,
    album: Option<String>,
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

    let album_param: Option<u64>;

    match album {
        Some(album_id) => {
            let album_id_res = parse_identifier(&album_id);

            match album_id_res {
                Ok(id) => {
                    album_param = Some(id);
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
                    eprintln!("Invalid album identifier specified.");
                    process::exit(1);
                }
            }
        }
        None => {
            album_param = None;
        }
    }

    let metadata_file = std::path::Path::new(&path)
        .join("metadata.json")
        .to_str()
        .unwrap()
        .to_string();

    let import_metadata: MediaMetadataExport;
    let metadata_file_res = tokio::fs::read_to_string(&metadata_file).await;

    match metadata_file_res {
        Ok(metadata_str) => {
            let parsed_metadata: Result<MediaMetadataExport, _> =
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

    // Upload

    let original_file_path: String = match import_metadata.original {
        Some(original_file) => std::path::Path::new(&path)
            .join(original_file)
            .to_str()
            .unwrap()
            .to_string(),
        None => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Invalid metadata: No original file specified.");
            process::exit(1);
        }
    };

    let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

    let upload_api_res = api_call_upload_media(
        &vault_url,
        original_file_path.clone(),
        import_metadata.title,
        album_param,
        global_opts.debug,
        progress_printer,
    )
    .await;

    let media_id: u64;
    let media_id_str: String;

    match upload_api_res {
        Ok(upload_res) => {
            media_id = upload_res.media_id;
            media_id_str = identifier_to_string(upload_res.media_id);

            eprintln!("Upload completed: {original_file_path}");
            eprintln!("Media asset created: {media_id_str}");
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

    // Wait for encryption

    let mut encryption_progress_printer = EncryptionProgressPrinter::new();

    encryption_progress_printer.progress_start();

    let mut encryption_done = false;

    while !encryption_done {
        let api_get_res = api_call_get_media(&vault_url, media_id, global_opts.debug).await;

        match api_get_res {
            Ok(media_data) => {
                encryption_done = media_data.ready;

                if !encryption_done {
                    encryption_progress_printer
                        .progress_update(media_data.ready_p.unwrap_or(0) as u64, 100);
                } else {
                    encryption_progress_printer.progress_update(100, 100);
                    encryption_progress_printer.progress_finish();
                }
            }
            Err(e) => {
                encryption_progress_printer.progress_finish();
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

    // Add tags

    if let Some(tags) = import_metadata.tags {
        for tag in tags {
            if global_opts.debug {
                eprintln!("Adding tag {tag} to {media_id_str}...");
            }

            let api_tag_res = api_call_tag_add(
                &vault_url,
                AddTagBody {
                    media_id,
                    tag_name: tag.clone(),
                },
                global_opts.debug,
            )
            .await;

            match api_tag_res {
                Ok(_) => {
                    eprintln!("Added tag {tag} to {media_id_str}");
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }

    // Set description

    if let Some(description) = import_metadata.description {
        if !description.is_empty() {
            let api_res = api_call_media_change_description(
                &vault_url,
                media_id,
                MediaUpdateDescriptionBody {
                    description: description.clone(),
                },
                global_opts.debug,
            )
            .await;

            match api_res {
                Ok(_) => {
                    let description_csv = to_csv_string(&description);

                    eprintln!(
                        "Successfully updated the description of {media_id_str}: {description_csv}"
                    );
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }

    // Set extra configuration

    if let Some(force_start_beginning) = import_metadata.force_start_beginning {
        if force_start_beginning {
            let api_res = api_call_media_change_extra(
                &vault_url,
                media_id,
                MediaUpdateExtraBody {
                    force_start_beginning,
                },
                global_opts.debug,
            )
            .await;

            match api_res {
                Ok(_) => {
                    eprintln!("Successfully updated the force-start-beginning param of {media_id_str}: {force_start_beginning}");
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
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

        let api_res = api_call_media_change_thumbnail(
            &vault_url,
            media_id,
            thumbnail_file_path.clone(),
            global_opts.debug,
            progress_printer,
        )
        .await;

        match api_res {
            Ok(upload_res) => {
                eprintln!("Upload completed: {thumbnail_file_path}");

                let thumb_new_url = upload_res.url;

                eprintln!("Successfully updated the thumbnail of {media_id_str}: {thumb_new_url}");
            }
            Err(e) => {
                print_request_error(e);
            }
        }
    }

    // Set extended description

    if let Some(ext_desc_file) = import_metadata.ext_desc {
        let ext_desc_file_path = std::path::Path::new(&path)
            .join(&ext_desc_file)
            .to_str()
            .unwrap()
            .to_string();

        let ext_desc_read_res = tokio::fs::read_to_string(&ext_desc_file_path).await;

        match ext_desc_read_res {
            Ok(ext_desc) => {
                let api_res = api_call_media_change_extended_description(
                    &vault_url,
                    media_id,
                    MediaUpdateExtendedDescriptionBody { ext_desc },
                    global_opts.debug,
                )
                .await;

                match api_res {
                    Ok(_) => {
                        eprintln!(
                            "Successfully updated the extended description of {media_id_str}"
                        );
                    }
                    Err(e) => {
                        print_request_error(e);
                    }
                }
            }
            Err(e) => {
                let e_str = e.to_string();
                eprintln!("Error reading the file {path}: {e_str}");
            }
        }
    }

    // Set time slices

    if let Some(time_slices) = import_metadata.time_slices {
        if !time_slices.is_empty() {
            let api_res = api_call_media_change_time_slices(
                &vault_url,
                media_id,
                time_slices,
                global_opts.debug,
            )
            .await;

            match api_res {
                Ok(_) => {
                    eprintln!("Successfully updated the time_slices of {media_id_str}");
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }

    // Set image notes

    if let Some(notes_file) = import_metadata.notes {
        let notes_file_path = std::path::Path::new(&path)
            .join(&notes_file)
            .to_str()
            .unwrap()
            .to_string();

        let image_notes_read_res = tokio::fs::read_to_string(&notes_file_path).await;

        match image_notes_read_res {
            Ok(image_notes_str) => {
                let parsed_notes_res: Result<Vec<ImageNote>, _> =
                    serde_json::from_str(&image_notes_str);

                match parsed_notes_res {
                    Ok(image_notes) => {
                        let api_res = api_call_media_change_notes(
                            &vault_url,
                            media_id,
                            image_notes,
                            global_opts.debug,
                        )
                        .await;

                        match api_res {
                            Ok(_) => {
                                eprintln!("Successfully updated the image notes of {media_id_str}");
                            }
                            Err(e) => {
                                print_request_error(e);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!(
                            "Error: The file {path} does not contain a valid set of image notes"
                        );
                    }
                }
            }
            Err(e) => {
                let e_str = e.to_string();
                eprintln!("Error reading the file {path}: {e_str}");
            }
        }
    }

    // Subtitles

    if let Some(subtitles) = import_metadata.subtitles {
        for subtitle in subtitles {
            let sub_id = subtitle.id.clone();
            let subtitle_file_path = std::path::Path::new(&path)
                .join(&subtitle.file)
                .to_str()
                .unwrap()
                .to_string();

            let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

            let api_res = api_call_media_set_subtitle(
                &vault_url,
                media_id,
                sub_id.clone(),
                subtitle.name.clone(),
                subtitle_file_path.clone(),
                global_opts.debug,
                progress_printer,
            )
            .await;

            match api_res {
                Ok(_) => {
                    eprintln!("Upload completed: {subtitle_file_path}");
                    eprintln!(
                        "Successfully uploaded new subtitles file for {media_id_str}: {sub_id}"
                    );
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }

    // Audios

    if let Some(audios) = import_metadata.audios {
        for audio in audios {
            let track_id = audio.id.clone();
            let audio_file_path = std::path::Path::new(&path)
                .join(&audio.file)
                .to_str()
                .unwrap()
                .to_string();

            let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

            let api_res = api_call_media_set_audio(
                &vault_url,
                media_id,
                track_id.clone(),
                audio.name.clone(),
                audio_file_path.clone(),
                global_opts.debug,
                progress_printer,
            )
            .await;

            match api_res {
                Ok(_) => {
                    eprintln!("Upload completed: {audio_file_path}");
                    eprintln!(
                        "Successfully uploaded new audio track file for {media_id_str}: {track_id}"
                    );
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }

    // Attachments

    if let Some(attachments) = import_metadata.attachments {
        for att in attachments {
            let att_name = att.name.clone();
            let att_file_path = std::path::Path::new(&path)
                .join(&att.file)
                .to_str()
                .unwrap()
                .to_string();

            let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

            let api_res = api_call_media_add_attachment(
                &vault_url,
                media_id,
                att_file_path.clone(),
                global_opts.debug,
                progress_printer,
            )
            .await;

            match api_res {
                Ok(uploaded_att) => {
                    eprintln!("Upload completed: {att_file_path}");

                    // Rename the attachment

                    let api_rename_res = api_call_media_rename_attachment(
                        &vault_url,
                        media_id,
                        MediaRenameAttachmentBody {
                            id: uploaded_att.id,
                            name: att.name.clone(),
                        },
                        global_opts.debug,
                    )
                    .await;

                    match api_rename_res {
                        Ok(_) => {
                            eprintln!(
                            "Successfully uploaded new attachment file for {media_id_str}: {att_name}"
                        );
                        }
                        Err(e) => {
                            print_request_error(e);
                        }
                    }
                }
                Err(e) => {
                    print_request_error(e);
                }
            }
        }
    }
}
