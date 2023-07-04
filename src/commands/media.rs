// Media command

use std::process;

use clap::Subcommand;

use crate::{
    api::{api_call_get_media, api_call_get_tags},
    commands::logout::do_logout,
    models::{tags_map_from_list, tags_names_from_ids},
    tools::{
        duration_to_string, ensure_login, format_date, identifier_to_string, parse_identifier,
        parse_vault_uri, to_csv_string,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum MediaCommand {
    /// Gets media asset metadata and download links
    Get {
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

    /// Uploads a new media asset
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
        #[arg(short, long)]
        tags: Option<String>,
    },
}

pub async fn run_media_cmd(global_opts: CommandGlobalOptions, cmd: MediaCommand) -> () {
    match cmd {
        MediaCommand::Get { media } => {
            run_cmd_get_media(global_opts, media).await;
        }
        MediaCommand::Download {
            media,
            asset,
            output,
            print_link,
        } => todo!(),
        MediaCommand::Upload {
            path,
            title,
            album,
            tags,
        } => todo!(),
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
            eprintln!("Invalid album identifier specified.");
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
