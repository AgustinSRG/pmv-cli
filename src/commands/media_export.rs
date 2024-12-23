// Media export command

use std::process;

use unicode_width::UnicodeWidthStr;

use crate::{
    api::{api_call_get_media, api_call_get_tags},
    commands::logout::do_logout,
    models::{tags_map_from_list, MediaAttachmentExport, MediaMetadata, MediaMetadataExport, MediaSubtitleOrAudioExport},
    tools::{
        ask_user, do_get_download_request, ensure_login, get_extension_from_url, parse_identifier, parse_vault_uri, ProgressReceiver, VaultURI
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_export_media(
    global_opts: CommandGlobalOptions,
    media: String,
    output: Option<String>,
    is_internal: bool,
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

    let media_id_res = parse_identifier(&media);
    let media_id: u64 = match media_id_res {
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

    // Get tags

    let tags_res = api_call_get_tags(&vault_url, global_opts.debug).await;

    if tags_res.is_err() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        print_request_error(tags_res.err().unwrap());
        process::exit(1);
    }

    let tags_vec = tags_res.unwrap();

    let tags_map = tags_map_from_list(&tags_vec);

    // Get media metadata

    let api_get_media_res =
        api_call_get_media(&vault_url, media_id, global_opts.debug).await;

    let media_metadata: MediaMetadata = match api_get_media_res {
        Ok(meta) => meta,
        Err(e) => {
            print_request_error(e);
            if is_internal {
                return;
            }
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
            format!("media_{media_id}")
        }
    };

    let out_exists = std::path::Path::new(&out_folder).exists();

    if out_exists && !global_opts.auto_confirm && !is_internal {
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

    let mut out_metadata = MediaMetadataExport {
        original: None,
        thumbnail: None,
        notes: None,
        ext_desc: None,
        title: None,
        description: None,
        tags: None,
        force_start_beginning: None,
        is_anim: None,
        time_slices: None,
        subtitles: None,
        audios: None,
        attachments: None,
    };

    out_metadata.title = Some(media_metadata.title.clone());
    out_metadata.description = Some(media_metadata.description.clone());
    out_metadata.force_start_beginning = media_metadata.force_start_beginning;
    out_metadata.is_anim = media_metadata.is_anim;
    out_metadata.time_slices = media_metadata.time_slices.clone();

    let mut tag_names_list: Vec<String> = Vec::new();

    for tag in media_metadata.tags {
        let tag_name_opt = tags_map.get(&tag);

        match tag_name_opt {
            Some(tag_name) => {
                tag_names_list.push(tag_name.clone());
            }
            None => {
                if global_opts.debug {
                    eprintln!("Warning: Skipped tag {tag} because it was not in the tag list.");
                }
            }
        }
    }

    out_metadata.tags = Some(tag_names_list.clone());

    // Original

    match media_metadata.url {
        Some(original_asset_url) => {
            if original_asset_url.is_empty() {
                eprintln!("The media has no original asset. It's probably still pending for upload or encryption.");
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

            let default_ext: String = match media_metadata.media_type {
                crate::models::MediaType::Deleted => "bin".to_string(),
                crate::models::MediaType::Image => "png".to_string(),
                crate::models::MediaType::Video => "mp4".to_string(),
                crate::models::MediaType::Audio => "mp3".to_string(),
            };

            let ext = get_extension_from_url(&original_asset_url, &default_ext);
            let out_file_name = "original".to_owned() + "." + &ext;

            let original_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                "original",
                original_asset_url,
                original_out_path,
                logout_after_operation,
            )
            .await;

            out_metadata.original = Some(out_file_name);
        }
        None => {
            eprintln!("The media has no original asset. It's probably still pending for upload or encryption.");
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

    // Thumbnail

    if !media_metadata.thumbnail.is_empty() {
        let ext = get_extension_from_url(&media_metadata.thumbnail, "jpg");
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
            media_metadata.thumbnail,
            thumbnail_out_path,
            logout_after_operation,
        )
        .await;

        out_metadata.thumbnail = Some(out_file_name);
    }

    // Extended description

    if let Some(ext_desc_url) = media_metadata.ext_desc_url {
        if !ext_desc_url.is_empty() {
            let ext = get_extension_from_url(&ext_desc_url, "txt");
            let out_file_name = "ext_desc".to_owned() + "." + &ext;

            let ext_desc_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                "extended description",
                ext_desc_url,
                ext_desc_out_path,
                logout_after_operation,
            )
            .await;

            out_metadata.ext_desc = Some(out_file_name);
        }
    }

    // Image notes

    if let Some(img_notes_url) = media_metadata.img_notes_url {
        if !img_notes_url.is_empty() {
            let ext = get_extension_from_url(&img_notes_url, "json");
            let out_file_name = "notes".to_owned() + "." + &ext;

            let notes_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                "image notes",
                img_notes_url,
                notes_out_path,
                logout_after_operation,
            )
            .await;

            out_metadata.notes = Some(out_file_name);
        }
    }

    // Subtitles

    if let Some(subtitles) = media_metadata.subtitles {
        let mut sub_counter = 0;
        let mut subtitles_export: Vec<MediaSubtitleOrAudioExport> = Vec::new();
        for subtitle in subtitles {
            sub_counter += 1;
            let sub_id = subtitle.id.clone();
            let d_name = format!("subtitle {sub_id}");
            let ext = get_extension_from_url(&subtitle.url, "srt");
            let out_file_name = format!("subtitle_{sub_counter}.{ext}");

            let sub_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                &d_name,
                subtitle.url,
                sub_out_path,
                logout_after_operation,
            )
            .await;

            subtitles_export.push(MediaSubtitleOrAudioExport {
                id: subtitle.id.clone(),
                name: subtitle.name.clone(),
                file: out_file_name,
            });
        }

        out_metadata.subtitles = Some(subtitles_export);
    }

    // Audio tracks

    if let Some(audios) = media_metadata.audios {
        let mut audio_counter = 0;
        let mut audios_export: Vec<MediaSubtitleOrAudioExport> = Vec::new();
        for audio in audios {
            audio_counter += 1;
            let audio_id = audio.id.clone();
            let d_name = format!("audio track {audio_id}");
            let ext = get_extension_from_url(&audio.url, "mp3");
            let out_file_name = format!("audio_track_{audio_counter}.{ext}");

            let audio_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                &d_name,
                audio.url,
                audio_out_path,
                logout_after_operation,
            )
            .await;

            audios_export.push(MediaSubtitleOrAudioExport {
                id: audio.id.clone(),
                name: audio.name.clone(),
                file: out_file_name,
            });
        }

        out_metadata.audios = Some(audios_export);
    }

    // Attachments

    if let Some(attachments) = media_metadata.attachments {
        let mut attachment_counter = 0;
        let mut attachments_export: Vec<MediaAttachmentExport> = Vec::new();
        for att in attachments {
            attachment_counter += 1;
            let att_name = att.name.clone();
            let d_name = format!("attachment {att_name}");
            let ext = get_extension_from_url(&att.url, "bin");
            let out_file_name = format!("attachment_{attachment_counter}.{ext}");

            let att_out_path = std::path::Path::new(&out_folder)
                .join(&out_file_name)
                .to_str()
                .unwrap()
                .to_string();

            download_media_asset(
                global_opts.clone(),
                &vault_url,
                &d_name,
                att.url,
                att_out_path,
                logout_after_operation,
            )
            .await;

            attachments_export.push(MediaAttachmentExport{
                name: att.name.clone(),
                file: out_file_name,
            });
        }

        out_metadata.attachments = Some(attachments_export);
    }

    // After everything is downloaded, write metadata

    let metadata_file = "metadata.json".to_string();
    let metadata_out_path = std::path::Path::new(&out_folder)
        .join(&metadata_file)
        .to_str()
        .unwrap()
        .to_string();

    let metadata_str = serde_json::to_string(&out_metadata).unwrap();

    let meta_write_res = tokio::fs::write(metadata_out_path.clone(), metadata_str).await;

    match meta_write_res {
        Ok(_) => {
            if logout_after_operation && !is_internal {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Done. Successfully exported media into folder {out_folder}");
        }
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
}

pub async fn download_media_asset(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    download_name: &str,
    download_path: String,
    out_file: String,
    logout_after_operation: bool,
) {
    let mut progress_printer = DownloaderProgressPrinter::new(download_name);

    let download_result = do_get_download_request(
        vault_url,
        download_path,
        out_file.clone(),
        global_opts.debug,
        &mut progress_printer,
    )
    .await;

    match download_result {
        Ok(_) => {
            eprintln!("Download completed: {out_file}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, vault_url).await;

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

struct DownloaderProgressPrinter {
    name: String,
    last_line_width: usize,
}

impl DownloaderProgressPrinter {
    fn new(name: &str) -> DownloaderProgressPrinter {
        DownloaderProgressPrinter {
            last_line_width: 0,
            name: name.to_string(),
        }
    }
}

impl ProgressReceiver for DownloaderProgressPrinter {
    fn progress_start(&mut self) {
        let name = &self.name;
        let line = format!("Downloading {name}...");
        eprint!("{line}");
        self.last_line_width = line.width();
    }

    fn progress_finish(&mut self) {
        eprintln!()
    }

    fn progress_update(&mut self, loaded: u64, total: u64) {
        let mut line: String;
        let name = &self.name;
        if total > 0 {
            let progress_percent: f64 = (loaded as f64) * 100.0 / (total as f64);
            line = format!(
                "Downloading {name}... {loaded} of {total} bytes. ({progress_percent:.2}%)"
            );
        } else {
            line = format!("Downloading {name}... {loaded} of unknown bytes.");
        }

        let line_width = line.width();

        if self.last_line_width > line_width {
            let pad = self.last_line_width - line_width;
            for _ in 0..pad {
                line.push(' ');
            }
        }

        eprint!("\r{line}");
        self.last_line_width = line.width();
    }
}
