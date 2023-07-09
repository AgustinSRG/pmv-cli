// Media download command

use std::process;

use unicode_width::UnicodeWidthStr;

use crate::{
    api::api_call_get_media,
    commands::logout::do_logout,
    models::{ConfigImageResolution, ConfigVideoResolution, TaskEncodeResolution},
    tools::{
        ask_user, do_get_download_request, ensure_login, parse_identifier, parse_vault_uri,
        ProgressReceiver, VaultURI,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Debug)]
pub enum DownloadAssetType {
    Original,
    Thumbnail,
    Resolution { width: i32, height: i32, fps: i32 },
    Subtitle(String),
    Audio(String),
    VideoPreview(u32),
    Notes,
}

pub fn parse_asset_type(s: &str) -> Result<DownloadAssetType, ()> {
    let parts: Vec<&str> = s.split(":").collect();
    if parts.is_empty() {
        return Err(());
    }

    let parts_type = parts[0].to_lowercase();
    let parts_value: Vec<&str> = parts.into_iter().skip(1).collect();
    let val = parts_value.join(":");

    if parts_type == "original" {
        return Ok(DownloadAssetType::Original);
    } else if parts_type == "thumbnail" {
        return Ok(DownloadAssetType::Thumbnail);
    } else if parts_type == "notes" {
        return Ok(DownloadAssetType::Notes);
    } else if parts_type == "resolution" || parts_type == "res" || parts_type == "r" {
        // Try video resolution
        let video_res = ConfigVideoResolution::from_str(&val);

        match video_res {
            Ok(r) => {
                return Ok(DownloadAssetType::Resolution {
                    width: r.width,
                    height: r.height,
                    fps: r.fps,
                });
            }
            Err(_) => {
                // Try image resolution
                let image_res = ConfigImageResolution::from_str(&val);

                match image_res {
                    Ok(r) => {
                        return Ok(DownloadAssetType::Resolution {
                            width: r.width,
                            height: r.height,
                            fps: 0,
                        });
                    }
                    Err(_) => {
                        return Err(());
                    }
                }
            }
        }
    } else if parts_type == "subtitle" || parts_type == "sub" || parts_type == "s" {
        if val.is_empty() {
            return Err(());
        } else {
            return Ok(DownloadAssetType::Subtitle(val.clone()));
        }
    } else if parts_type == "audio" || parts_type == "a" {
        if val.is_empty() {
            return Err(());
        } else {
            return Ok(DownloadAssetType::Audio(val.clone()));
        }
    } else if parts_type == "preview" || parts_type == "pre" || parts_type == "p" {
        let i_res = val.parse::<u32>();
        match i_res {
            Ok(i) => {
                return Ok(DownloadAssetType::VideoPreview(i));
            }
            Err(_) => {
                return Err(());
            }
        }
    } else {
        return Err(());
    }
}

pub async fn run_cmd_download_media(
    global_opts: CommandGlobalOptions,
    media: String,
    asset: Option<String>,
    output: Option<String>,
    print_link: bool,
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

    let asset_type;

    match asset {
        Some(asset_str) => {
            let asset_parsed = parse_asset_type(&asset_str);

            match asset_parsed {
                Ok(t) => {
                    asset_type = t;
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
                    eprintln!("Invalid asset type: {asset_str}");
                    process::exit(1);
                }
            }
        }
        None => {
            asset_type = DownloadAssetType::Original;
        }
    }

    // Call API

    let api_res = api_call_get_media(vault_url.clone(), media_id, global_opts.debug).await;

    match api_res {
        Ok(media_data) => {
            let mut download_path: String = "".to_string();

            match asset_type {
                DownloadAssetType::Original => match media_data.url {
                    Some(u) => {
                        if u.is_empty() {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("Original asset is not ready");
                            process::exit(1);
                        }

                        download_path = u;
                    }
                    None => {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("Original asset is not ready");
                        process::exit(1);
                    }
                },
                DownloadAssetType::Thumbnail => {
                    if media_data.thumbnail.is_empty() {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("This media asset has no thumbnail");
                        process::exit(1);
                    }

                    download_path = media_data.thumbnail;
                }
                DownloadAssetType::Resolution { width, height, fps } => {
                    match media_data.resolutions {
                        Some(resolutions) => {
                            if resolutions.is_empty() {
                                if logout_after_operation {
                                    let logout_res =
                                        do_logout(global_opts.clone(), vault_url.clone()).await;

                                    match logout_res {
                                        Ok(_) => {}
                                        Err(_) => {
                                            process::exit(1);
                                        }
                                    }
                                }
                                eprintln!("This media asset has no resolutions");
                                process::exit(1);
                            }

                            let mut found = false;

                            for resolution in resolutions {
                                let resolution_fps = resolution.fps.unwrap_or(0);
                                if fps > 0 && resolution_fps > 0 {
                                    if width != resolution.width
                                        || height != resolution.height
                                        || fps != resolution_fps
                                    {
                                        continue;
                                    }
                                } else if fps <= 0 && resolution_fps <= 0 {
                                    if width != resolution.width || height != resolution.height {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }

                                match resolution.url {
                                    Some(u) => {
                                        if u.is_empty() {
                                            if logout_after_operation {
                                                let logout_res = do_logout(
                                                    global_opts.clone(),
                                                    vault_url.clone(),
                                                )
                                                .await;

                                                match logout_res {
                                                    Ok(_) => {}
                                                    Err(_) => {
                                                        process::exit(1);
                                                    }
                                                }
                                            }
                                            let task_res = TaskEncodeResolution {
                                                width: width,
                                                height: height,
                                                fps: fps,
                                            };
                                            let task_res_str = task_res.to_string();
                                            eprintln!(
                                                "No resolution {task_res_str} is not ready yet"
                                            );
                                            process::exit(1);
                                        }

                                        download_path = u;
                                        found = true;
                                    }
                                    None => {
                                        if logout_after_operation {
                                            let logout_res =
                                                do_logout(global_opts.clone(), vault_url.clone())
                                                    .await;

                                            match logout_res {
                                                Ok(_) => {}
                                                Err(_) => {
                                                    process::exit(1);
                                                }
                                            }
                                        }
                                        let task_res = TaskEncodeResolution {
                                            width: width,
                                            height: height,
                                            fps: fps,
                                        };
                                        let task_res_str = task_res.to_string();
                                        eprintln!("No resolution {task_res_str} is not ready yet");
                                        process::exit(1);
                                    }
                                }

                                break;
                            }

                            if !found {
                                if logout_after_operation {
                                    let logout_res =
                                        do_logout(global_opts.clone(), vault_url.clone()).await;

                                    match logout_res {
                                        Ok(_) => {}
                                        Err(_) => {
                                            process::exit(1);
                                        }
                                    }
                                }
                                let task_res = TaskEncodeResolution {
                                    width: width,
                                    height: height,
                                    fps: fps,
                                };
                                let task_res_str = task_res.to_string();
                                eprintln!("No resolution found matching {task_res_str}");
                                process::exit(1);
                            }
                        }
                        None => {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("This media asset has no resolutions");
                            process::exit(1);
                        }
                    }
                }
                DownloadAssetType::Subtitle(id) => match media_data.subtitles {
                    Some(subtitles) => {
                        if subtitles.is_empty() {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("This media asset has no subtitles");
                            process::exit(1);
                        }

                        let mut found = false;

                        for subtitle in subtitles {
                            if subtitle.id.to_lowercase() != id.to_lowercase() {
                                continue;
                            }

                            if subtitle.url.is_empty() {
                                if logout_after_operation {
                                    let logout_res =
                                        do_logout(global_opts.clone(), vault_url.clone()).await;

                                    match logout_res {
                                        Ok(_) => {}
                                        Err(_) => {
                                            process::exit(1);
                                        }
                                    }
                                }
                                eprintln!("The subtitle is not ready yet");
                                process::exit(1);
                            }

                            download_path = subtitle.url;
                            found = true;
                            break;
                        }

                        if !found {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("No subtitle found matching {id}");
                            process::exit(1);
                        }
                    }
                    None => {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("This media asset has no subtitles");
                        process::exit(1);
                    }
                },
                DownloadAssetType::Audio(id) => match media_data.audios {
                    Some(audios) => {
                        if audios.is_empty() {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("This media asset has no audio tracks");
                            process::exit(1);
                        }

                        let mut found = false;

                        for audio in audios {
                            if audio.id.to_lowercase() != id.to_lowercase() {
                                continue;
                            }

                            if audio.url.is_empty() {
                                if logout_after_operation {
                                    let logout_res =
                                        do_logout(global_opts.clone(), vault_url.clone()).await;

                                    match logout_res {
                                        Ok(_) => {}
                                        Err(_) => {
                                            process::exit(1);
                                        }
                                    }
                                }
                                eprintln!("The audio track is not ready yet");
                                process::exit(1);
                            }

                            download_path = audio.url;
                            found = true;
                            break;
                        }

                        if !found {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("No audio track found matching {id}");
                            process::exit(1);
                        }
                    }
                    None => {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("This media asset has no audio tracks");
                        process::exit(1);
                    }
                },
                DownloadAssetType::VideoPreview(i) => match media_data.video_previews {
                    Some(u) => {
                        if u.is_empty() {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("This media asset has no video previews");
                            process::exit(1);
                        }

                        let index_str = i.to_string();

                        download_path = u.replace("{INDEX}", &index_str)
                    }
                    None => {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("This media asset has no video previews");
                        process::exit(1);
                    }
                },
                DownloadAssetType::Notes => match media_data.img_notes_url {
                    Some(u) => {
                        if u.is_empty() {
                            if logout_after_operation {
                                let logout_res =
                                    do_logout(global_opts.clone(), vault_url.clone()).await;

                                match logout_res {
                                    Ok(_) => {}
                                    Err(_) => {
                                        process::exit(1);
                                    }
                                }
                            }
                            eprintln!("This media asset has no image notes");
                            process::exit(1);
                        }

                        download_path = u;
                    }
                    None => {
                        if logout_after_operation {
                            let logout_res =
                                do_logout(global_opts.clone(), vault_url.clone()).await;

                            match logout_res {
                                Ok(_) => {}
                                Err(_) => {
                                    process::exit(1);
                                }
                            }
                        }
                        eprintln!("This media asset has no image notes");
                        process::exit(1);
                    }
                },
            }

            if download_path.is_empty() {
                if logout_after_operation {
                    let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }
                eprintln!("Could not find the specified asset");
                process::exit(1);
            }

            if print_link {
                if logout_after_operation {
                    let logout_res = do_logout(global_opts, vault_url.clone()).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }

                let download_link = vault_url.resolve_asset(&download_path);
                println!("{download_link}");
            } else {
                download_media_asset(
                    global_opts,
                    vault_url,
                    download_path,
                    output,
                    logout_after_operation,
                )
                .await;
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

pub async fn download_media_asset(
    global_opts: CommandGlobalOptions,
    vault_url: VaultURI,
    download_path: String,
    output: Option<String>,
    logout_after_operation: bool,
) {
    // Find the output file

    let out_file: String;

    match output {
        Some(file) => {
            if file.is_empty() {
                let path_parts: Vec<&str> = download_path.split("/").collect();

                if path_parts.is_empty() {
                    out_file = "download".to_string();
                } else {
                    let last_part = path_parts.into_iter().last().unwrap_or("download");
                    out_file = last_part
                        .split("?")
                        .into_iter()
                        .nth(0)
                        .unwrap_or("download")
                        .to_string();
                }
            } else {
                out_file = file;
            }
        }
        None => {
            let path_parts: Vec<&str> = download_path.split("/").collect();

            if path_parts.is_empty() {
                out_file = "download".to_string();
            } else {
                let last_part = path_parts.into_iter().last().unwrap_or("download");
                out_file = last_part
                    .split("?")
                    .into_iter()
                    .nth(0)
                    .unwrap_or("download")
                    .to_string();
            }
        }
    }

    let out_exists = std::path::Path::new(&out_file).exists();

    if out_exists && !global_opts.auto_confirm {
        eprintln!("The file {out_file} already exists");
        let confirmation = ask_user("Do you want to overwrite it? y/n: ")
            .await
            .unwrap_or("".to_string());

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

    let mut progress_printer = DownloaderProgressPrinter::new();

    let download_result = do_get_download_request(
        vault_url.clone(),
        download_path,
        out_file.clone(),
        global_opts.debug,
        &mut progress_printer,
    )
    .await;

    if logout_after_operation {
        let logout_res = do_logout(global_opts, vault_url.clone()).await;

        match logout_res {
            Ok(_) => {}
            Err(_) => {
                process::exit(1);
            }
        }
    }

    match download_result {
        Ok(_) => {
            eprintln!("Download completed: {out_file}");
        }
        Err(e) => {
            print_request_error(e);
            process::exit(1);
        }
    }
}

struct DownloaderProgressPrinter {
    last_line_width: usize,
}

impl DownloaderProgressPrinter {
    fn new() -> DownloaderProgressPrinter {
        return DownloaderProgressPrinter {
            last_line_width: 0,
        };
    }
}

impl ProgressReceiver for DownloaderProgressPrinter {
    fn progress_start(self: &mut Self) -> () {
        let line = "Downloading...".to_string();
        eprint!("{line}");
        self.last_line_width = line.width();
    }

    fn progress_finish(self: &mut Self) -> () {
        eprint!("\n")
    }

    fn progress_update(self: &mut Self, loaded: u64, total: u64) -> () {
        let mut line: String;
        if total > 0 {
            let progress_percent: f64 = (loaded as f64) * 100.0 / (total as f64);
            line = format!("Downloading... {loaded} of {total} bytes. ({progress_percent:.2}%)");
        } else {
            line = format!("Downloading... {loaded} of unknown bytes.");
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