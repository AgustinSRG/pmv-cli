// Media command

use std::process;

use clap::Subcommand;

use crate::{
    api::{
        api_call_get_media, api_call_get_media_stats, api_call_get_tags,
        api_call_media_change_description, api_call_media_change_extra,
        api_call_media_change_title, api_call_media_re_encode, api_call_media_delete,
    },
    commands::logout::do_logout,
    models::{
        tags_map_from_list, tags_names_from_ids, MediaUpdateDescriptionBody, MediaUpdateExtraBody,
        MediaUpdateTitleBody,
    },
    tools::{
        duration_to_string, ensure_login, format_date, identifier_to_string, parse_identifier,
        parse_vault_uri, render_size_bytes, to_csv_string, ask_user,
    },
};

use super::{
    get_vault_url, media_download::run_cmd_download_media,
    media_thumbnail::run_cmd_upload_media_thumbnail, media_upload::run_cmd_upload_media,
    print_request_error, CommandGlobalOptions, media_time_slices::{run_cmd_get_media_time_slices, run_cmd_set_media_time_slices}, media_image_notes::run_cmd_set_media_image_notes,
};

#[derive(Subcommand)]
pub enum MediaCommand {
    /// Gets media asset metadata and download links
    Get {
        /// Media asset ID
        media: String,
    },

    /// Gets media asset size stats
    Stats {
        /// Media asset ID
        media: String,
    },

    /// Downloads a media asset
    Download {
        /// Media asset ID
        media: String,

        /// Asset to download. Examples: original, thumbnail, resolution:1280x720:30, sub:ID, audio:ID, notes, preview:Index
        asset: Option<String>,

        /// Path to the file to download the asset into
        #[arg(short, long)]
        output: Option<String>,

        /// Prints the download link, instead of downloading to a file
        #[arg(short, long)]
        print_link: bool,
    },

    /// Uploads a new media asset, waits for encryption and adds tags if specified
    Upload {
        /// Path to the file to upload
        path: String,

        /// A title for the media asset
        #[arg(short, long)]
        title: Option<String>,

        /// Album to upload the media asset into
        #[arg(short, long)]
        album: Option<String>,

        /// Tags to add to the media asset, separated by spaces.
        #[arg(short = 'T', long)]
        tags: Option<String>,

        /// Do not wait for encryption
        #[arg(short, long)]
        skip_encryption: bool,
    },

    /// Changes the title of a media asset
    SetTitle {
        /// Media asset ID
        media: String,

        /// Title
        title: String,
    },

    /// Changes the description of a media asset
    SetDescription {
        /// Media asset ID
        media: String,

        /// Description
        description: String,
    },

    /// Changes the description of a media asset
    SetForceStartBeginning {
        /// Media asset ID
        media: String,

        /// Set to 'true' if you want to tell the clients not to store the time, so they always start from the beginning
        force_start_beginning: String,
    },

    /// Sets the thumbnail of a media asset
    SetThumbnail {
        /// Media asset ID
        media: String,

        /// Path to the thumbnail file
        path: String,
    },

    /// Prints the time slices of a media asset
    GetTimeSlices {
        /// Media asset ID
        media: String,
    },

    /// Sets the time slices of a media asset
    SetTimeSlices {
        /// Media asset ID
        media: String,

        /// Path to the file containing the time slices
        path: String,
    },

    /// Sets the image notes of a media asset
    SetImageNotes {
        /// Media asset ID
        media: String,

        /// Path to the image notes file
        path: String,
    },

    /// Adds new resolution to the media asset
    AddResolution {
        /// Media asset ID
        media: String,

        /// Resolution. Example: 1280x720:30
        resolution: String,
    },

    /// Removes a resolution from the media asset
    RemoveResolution {
        /// Media asset ID
        media: String,

        /// Resolution. Example: 1280x720:30
        resolution: String,
    },

    /// Adds subtitle file to a media asset
    AddSubtitle {
        /// Media asset ID
        media: String,

        /// Subtitle file identifier. Example: EN
        sub_id: String,

        /// Path to the subtitles file
        path: String,

        /// Subtitle file display name. If not specified, the identifier is used.
        #[arg(long)]
        name: Option<String>,
    },

    /// Removes subtitle file from a media asset
    RemoveSubtitle {
        /// Media asset ID
        media: String,

        /// Subtitle file identifier. Example: EN
        sub_id: String,
    },

    /// Adds audio track file to a media asset
    AddAudio {
        /// Media asset ID
        media: String,

        /// Audio track file identifier. Example: EN
        sub_id: String,

        /// Path to the audio track file
        path: String,

        /// Audio track file display name. If not specified, the identifier is used.
        #[arg(long)]
        name: Option<String>,
    },

    /// Removes audio track file from a media asset
    RemoveAudio {
        /// Media asset ID
        media: String,

        /// Audio track file identifier. Example: EN
        sub_id: String,
    },

    /// Re-Encodes a media asset
    ReEncode {
        /// Media asset ID
        media: String,
    },

    /// Deletes a media asset
    Delete {
        /// Media asset ID
        media: String,
    },
}

pub async fn run_media_cmd(global_opts: CommandGlobalOptions, cmd: MediaCommand) -> () {
    match cmd {
        MediaCommand::Get { media } => {
            run_cmd_get_media(global_opts, media).await;
        }
        MediaCommand::Stats { media } => {
            run_cmd_get_media_stats(global_opts, media).await;
        }
        MediaCommand::Download {
            media,
            asset,
            output,
            print_link,
        } => {
            run_cmd_download_media(global_opts, media, asset, output, print_link).await;
        }
        MediaCommand::Upload {
            path,
            title,
            album,
            tags,
            skip_encryption,
        } => {
            run_cmd_upload_media(global_opts, path, title, album, tags, skip_encryption).await;
        }
        MediaCommand::SetTitle { media, title } => {
            run_cmd_media_set_title(global_opts, media, title).await;
        }
        MediaCommand::SetDescription { media, description } => {
            run_cmd_media_set_description(global_opts, media, description).await;
        }
        MediaCommand::SetForceStartBeginning {
            media,
            force_start_beginning,
        } => {
            run_cmd_media_set_force_start_beginning(global_opts, media, force_start_beginning)
                .await;
        }
        MediaCommand::SetThumbnail { media, path } => {
            run_cmd_upload_media_thumbnail(global_opts, media, path).await;
        }
        MediaCommand::GetTimeSlices { media } => {
            run_cmd_get_media_time_slices(global_opts, media).await;
        }
        MediaCommand::SetTimeSlices { media, path } => {
            run_cmd_set_media_time_slices(global_opts, media, path).await;
        }
        MediaCommand::SetImageNotes { media, path } => {
            run_cmd_set_media_image_notes(global_opts, media, path).await;
        }
        MediaCommand::AddResolution { media, resolution } => todo!(),
        MediaCommand::RemoveResolution { media, resolution } => todo!(),
        MediaCommand::AddSubtitle {
            media,
            sub_id,
            path,
            name,
        } => todo!(),
        MediaCommand::RemoveSubtitle { media, sub_id } => todo!(),
        MediaCommand::AddAudio {
            media,
            sub_id,
            path,
            name,
        } => todo!(),
        MediaCommand::RemoveAudio { media, sub_id } => todo!(),
        MediaCommand::ReEncode { media } => {
            run_cmd_media_re_encode(global_opts, media).await;
        }
        MediaCommand::Delete { media } => {
            run_cmd_media_delete(global_opts, media).await;
        }
    }
}

pub async fn run_cmd_get_media(global_opts: CommandGlobalOptions, media: String) -> () {
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

    // Get tags

    let tags_res = api_call_get_tags(vault_url.clone(), global_opts.debug).await;

    if tags_res.is_err() {
        if logout_after_operation {
            let logout_res = do_logout(global_opts, vault_url.clone()).await;

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

    let tags = tags_map_from_list(&tags_res.unwrap());

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

            let out_id = identifier_to_string(media_data.id);
            println!("ID: {out_id}");

            let out_type = media_data.media_type.to_string();
            println!("Type: {out_type}");

            match media_data.media_type {
                crate::models::MediaType::Deleted => {}
                crate::models::MediaType::Image => {
                    let width = media_data.width.unwrap_or(0);
                    let height = media_data.height.unwrap_or(0);

                    println!("Size: {width}x{height}");
                }
                crate::models::MediaType::Video => {
                    let width = media_data.width.unwrap_or(0);
                    let height = media_data.height.unwrap_or(0);

                    println!("Size: {width}x{height}");

                    let fps = media_data.fps.unwrap_or(0);

                    println!("Frames per second: {fps}");

                    let duration = duration_to_string(media_data.duration.unwrap_or(0.0));

                    println!("Duration: {duration}");
                }
                crate::models::MediaType::Audio => {
                    let duration = duration_to_string(media_data.duration.unwrap_or(0.0));

                    println!("Duration: {duration}");
                }
            }

            match media_data.force_start_beginning {
                Some(force_start_beginning) => {
                    if force_start_beginning {
                        println!("Force start beginning: ENABLED");
                    }
                }
                None => {}
            }

            let out_upload_date = format_date(media_data.upload_time);
            println!("Upload date: {out_upload_date}");

            let out_title = to_csv_string(&media_data.title);
            println!("Title: {out_title}");

            let out_description = to_csv_string(&media_data.description);
            println!("Description: {out_description}");

            if !media_data.thumbnail.is_empty() {
                let out_thumbnail = media_data.thumbnail;
                println!("Thumbnail: {out_thumbnail}");
            } else {
                println!("Thumbnail: None");
            }

            if !media_data.tags.is_empty() {
                let tag_list = tags_names_from_ids(&media_data.tags, &tags).join(" ");
                println!("Tags: {tag_list}");
            }

            if media_data.ready {
                if media_data.encoded {
                    println!("Status: Encoded and ready");
                    match media_data.url {
                        Some(original_url) => {
                            println!("Original: {original_url}");
                        }
                        None => {}
                    }
                } else {
                    println!("Status: Not encoded yet");
                    match media_data.task {
                        Some(t) => {
                            let task_id = identifier_to_string(t);

                            println!("Task (Encode): {task_id}");
                        }
                        None => {
                            println!("No task to encode the media. Re-encoding may be needed.");
                        }
                    }
                }
            } else {
                println!("Status: Not ready");
            }

            match media_data.resolutions {
                Some(resolutions) => {
                    if !resolutions.is_empty() {
                        println!("Extra resolutions:");

                        for resolution in resolutions {
                            let res_str = resolution.to_string();
                            println!("\t- Resolution: {res_str}");

                            if resolution.ready {
                                println!("\t  Status: Ready");
                                match resolution.url {
                                    Some(resolution_url) => {
                                        println!("\t  File: {resolution_url}");
                                    }
                                    None => {}
                                }
                            } else {
                                println!("\t  Status: Not ready yet");
                                match resolution.task {
                                    Some(t) => {
                                        let task_id = identifier_to_string(t);

                                        println!("\t  Task (Encode): {task_id}");
                                    }
                                    None => {
                                        println!(
                                            "\t  No task set to encode. Re-encoding may be needed."
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                None => {}
            }

            match media_data.audios {
                Some(audios) => {
                    if !audios.is_empty() {
                        println!("Audio tracks:");

                        for audio in audios {
                            let audio_id = to_csv_string(&audio.id);
                            println!("\t- Audio ID: {audio_id}");
                            let audio_name = to_csv_string(&audio.name);
                            println!("\t  Name: {audio_name}");
                            let url = audio.url;
                            println!("\t  File: {url}");
                        }
                    }
                }
                None => {}
            }

            match media_data.subtitles {
                Some(subtitles) => {
                    if !subtitles.is_empty() {
                        println!("Subtitles:");

                        for sub in subtitles {
                            let sub_id = to_csv_string(&sub.id);
                            println!("\t- Subtitle ID: {sub_id}");
                            let sub_name = to_csv_string(&sub.name);
                            println!("\t  Name: {sub_name}");
                            let url = sub.url;
                            println!("\t  File: {url}");
                        }
                    }
                }
                None => {}
            }

            match media_data.img_notes {
                Some(_) => match media_data.img_notes_url {
                    Some(img_notes_url) => {
                        if !img_notes_url.is_empty() {
                            println!("Image notes: {img_notes_url}");
                        }
                    }
                    None => {}
                },
                None => {}
            }

            match media_data.time_slices {
                Some(time_slices) => {
                    if !time_slices.is_empty() {
                        println!("Time slices:");

                        for time_slice in time_slices {
                            let time_slice_str = duration_to_string(time_slice.time);
                            let time_slice_name = time_slice.name;

                            println!("\t- {time_slice_str} - {time_slice_name}");
                        }
                    }
                }
                None => {}
            }

            match media_data.video_previews {
                Some(video_previews_uri) => match media_data.video_previews_interval {
                    Some(video_previews_interval) => {
                        if !video_previews_uri.is_empty() {
                            let video_previews_interval_str =
                                duration_to_string(video_previews_interval);
                            println!("Video previews (Interval: {video_previews_interval_str}): {video_previews_uri}");
                        }
                    }
                    None => {}
                },
                None => {}
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

pub async fn run_cmd_get_media_stats(global_opts: CommandGlobalOptions, media: String) -> () {
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

    let api_res = api_call_get_media_stats(vault_url.clone(), media_id, global_opts.debug).await;

    match api_res {
        Ok(stats) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let meta_size = render_size_bytes(stats.meta_size);

            println!("Metadata size: {meta_size}");
            println!("Assets:");

            for asset in stats.assets {
                let asset_name = asset.name;
                let size = render_size_bytes(asset.size);
                println!("\t- {asset_name}: {size}");
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

pub async fn run_cmd_media_set_title(
    global_opts: CommandGlobalOptions,
    media: String,
    title: String,
) -> () {
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

    // Call API

    let api_res = api_call_media_change_title(
        vault_url.clone(),
        media_id_param,
        MediaUpdateTitleBody {
            title: title.clone(),
        },
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

            let title_csv = to_csv_string(&title);

            eprintln!("Successfully updated the title of #{media_id_param}: {title_csv}");
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

pub async fn run_cmd_media_set_description(
    global_opts: CommandGlobalOptions,
    media: String,
    description: String,
) -> () {
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

    // Call API

    let api_res = api_call_media_change_description(
        vault_url.clone(),
        media_id_param,
        MediaUpdateDescriptionBody {
            description: description.clone(),
        },
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

            let description_csv = to_csv_string(&description);

            eprintln!(
                "Successfully updated the description of #{media_id_param}: {description_csv}"
            );
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

pub async fn run_cmd_media_set_force_start_beginning(
    global_opts: CommandGlobalOptions,
    media: String,
    force_start_beginning: String,
) -> () {
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

    // Param

    let force_start_beginning_lower = force_start_beginning.to_lowercase();
    let force_start_beginning_bool: bool;

    if force_start_beginning_lower == "true"
        || force_start_beginning_lower == "yes"
        || force_start_beginning_lower == "1"
    {
        force_start_beginning_bool = true;
    } else if force_start_beginning_lower == "false"
        || force_start_beginning_lower == "no"
        || force_start_beginning_lower == "0"
    {
        force_start_beginning_bool = false;
    } else {
        if logout_after_operation {
            let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Invalid FORCE_START_BEGINNING parameter. Set it to 'true' or 'false'.");
        process::exit(1);
    }

    // Call API

    let api_res = api_call_media_change_extra(
        vault_url.clone(),
        media_id_param,
        MediaUpdateExtraBody {
            force_start_beginning: force_start_beginning_bool,
        },
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

            eprintln!("Successfully updated the force-start-beginning param of #{media_id_param}: {force_start_beginning_bool}");
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

pub async fn run_cmd_media_re_encode(global_opts: CommandGlobalOptions, media: String) -> () {
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

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to re-encode the media asset {media_id_param}?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

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

    // Call API

    let api_res = api_call_media_re_encode(
        vault_url.clone(),
        media_id_param,
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

            eprintln!("Successfully requested media asset #{media_id_param} to be re-encoded");
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

pub async fn run_cmd_media_delete(global_opts: CommandGlobalOptions, media: String) -> () {
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

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to delete the media asset {media_id_param}?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

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

    // Call API

    let api_res = api_call_media_delete(
        vault_url.clone(),
        media_id_param,
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

            eprintln!("Successfully deleted asset #{media_id_param}");
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
