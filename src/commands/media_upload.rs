// Media upload command

use std::{
    process,
    sync::{Arc, Mutex},
};

use unicode_width::UnicodeWidthStr;

use crate::{
    api::{api_call_get_media, api_call_upload_media, api_call_tag_add},
    commands::logout::do_logout,
    models::{parse_tag_name, AddTagBody},
    tools::{
        ensure_login, identifier_to_string, parse_identifier, parse_vault_uri, ProgressReceiver,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_upload_media(
    global_opts: CommandGlobalOptions,
    path: String,
    title: Option<String>,
    album: Option<String>,
    tags: Option<String>,
    skip_encryption: bool,
) {
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
        }
        None => {
            album_param = None;
        }
    }

    // Upload progress reporter

    let progress_printer = Arc::new(Mutex::new(UploaderProgressPrinter::new()));

    let api_res = api_call_upload_media(
        vault_url.clone(),
        path.clone(),
        title,
        album_param,
        global_opts.debug,
        progress_printer,
    )
    .await;

    match api_res {
        Ok(upload_res) => {
            let media_id = identifier_to_string(upload_res.media_id);

            eprintln!("Upload completed: {path}");
            eprintln!("Media asset created: {media_id}");

            // Wait for encryption

            if !skip_encryption {
                let mut encryption_progress_printer = EncryptionProgressPrinter::new();

                encryption_progress_printer.progress_start();
    
                let mut encryption_done = false;
    
                while !encryption_done {
                    let api_get_res =
                        api_call_get_media(vault_url.clone(), upload_res.media_id, global_opts.debug).await;
    
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
                                let logout_res = do_logout(global_opts, vault_url.clone()).await;
    
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
            }

            // Add tags

            let tags_param: Vec<String> = tags
                .unwrap_or("".to_string())
                .split(" ")
                .map(|t| parse_tag_name(t))
                .filter(|t| !t.is_empty())
                .collect();

            if !tags_param.is_empty() {
                for tag in tags_param {
                    if global_opts.debug {
                        eprintln!("Adding tag {tag} to {media_id}...");
                    }

                    let api_tag_res = api_call_tag_add(
                        vault_url.clone(),
                        AddTagBody {
                            media_id: upload_res.media_id,
                            tag_name: tag.clone(),
                        },
                        global_opts.debug,
                    )
                    .await;

                    match api_tag_res {
                        Ok(_) => {
                            eprintln!("Added tag {tag} to {media_id}");
                        }
                        Err(e) => {
                            print_request_error(e);
                        }
                    }
                }
            }

            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            eprintln!("Done: {media_id}")
        }
        Err(e) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

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

pub struct UploaderProgressPrinter {
    last_line_width: usize,
}

impl UploaderProgressPrinter {
    pub fn new() -> UploaderProgressPrinter {
        return UploaderProgressPrinter { last_line_width: 0 };
    }
}

impl ProgressReceiver for UploaderProgressPrinter {
    fn progress_start(self: &mut Self) -> () {
        let line = "Uploading...".to_string();
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
            line = format!("Uploading... {loaded} of {total} bytes. ({progress_percent:.2}%)");
        } else {
            line = format!("Uploading... {loaded} of unknown bytes.");
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

pub struct EncryptionProgressPrinter {
    last_line_width: usize,
}

impl EncryptionProgressPrinter {
    pub fn new() -> EncryptionProgressPrinter {
        return EncryptionProgressPrinter { last_line_width: 0 };
    }
}

impl ProgressReceiver for EncryptionProgressPrinter {
    fn progress_start(self: &mut Self) -> () {
        let line = "Encrypting...".to_string();
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
            line = format!("Encrypting... ({progress_percent:.2}%)");
        } else {
            line = format!("Encrypting...");
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
