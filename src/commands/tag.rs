// Tab command

use std::process;

use clap::Subcommand;
use hyper::StatusCode;

use crate::{
    api::{api_call_get_media, api_call_get_tags, api_call_tag_add, api_call_tag_remove},
    commands::logout::do_logout,
    models::{parse_tag_name, tags_map_from_list, tags_reverse_map_from_list, AddTagBody, RemoveTagBody},
    tools::{
        ensure_login, is_identifier, parse_identifier, parse_vault_uri, print_table, to_csv_string, identifier_to_string,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum TagCommand {
    /// List tags
    #[clap(alias("ls"))]
    List {
        /// CSV format
        #[arg(short, long)]
        csv: bool,

        /// Sort alphabetically by name
        #[arg(short, long)]
        alphabetically: bool,
    },

    /// Adds a tag to a media asset
    Add {
        /// Tag name or identifier
        tag: String,

        /// Media asset ID
        media: String,
    },

    /// Removes a tag from a media asset
    Remove {
        /// Tag name or identifier
        tag: String,

        /// Media asset ID
        media: String,
    },
}

pub async fn run_tag_cmd(global_opts: CommandGlobalOptions, cmd: TagCommand) -> () {
    match cmd {
        TagCommand::List { csv, alphabetically } => {
            run_cmd_list_tags(global_opts, csv, alphabetically).await;
        }
        TagCommand::Add { tag, media } => {
            run_cmd_tag_add(global_opts, tag, media).await;
        }
        TagCommand::Remove { tag, media } => {
            run_cmd_tag_remove(global_opts, tag, media).await;
        }
    }
}

pub async fn run_cmd_list_tags(global_opts: CommandGlobalOptions, csv: bool, alphabetically: bool) -> () {
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

    // Call API

    let api_res = api_call_get_tags(vault_url.clone(), global_opts.debug).await;

    match api_res {
        Ok(mut tags) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            // Sort

            if alphabetically {
                tags.sort_by(|a, b| a.name.cmp(&b.name));
            } else {
                tags.sort_by(|a, b| a.id.cmp(&b.id));
            }

            let total = tags.len();

            println!("total: {total}");

            if csv {
                println!("");
                println!("\"Tag Id\",\"Tag Name\"");

                for tag in tags {
                    let row_id = identifier_to_string(tag.id);
                    let row_name = to_csv_string(&tag.name);
                    println!("{row_id},{row_name}");
                }
            } else {
                let table_head: Vec<String> = vec!["Tag Id".to_string(), "Tag Name".to_string()];

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(total);

                for tag in tags {
                    table_body.push(vec![identifier_to_string(tag.id), to_csv_string(&tag.name)]);
                }

                print_table(&table_head, &table_body);
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

pub async fn run_cmd_tag_add(global_opts: CommandGlobalOptions, tag: String, media: String) -> () {
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
                    match e {
                        crate::tools::RequestError::StatusCodeError(_)
                        | crate::tools::RequestError::HyperError(_)
                        | crate::tools::RequestError::JSONError {
                            message: _,
                            body: _,
                        } => {
                            print_request_error(e);
                        }
                        crate::tools::RequestError::ApiError {
                            status,
                            code: _,
                            message: _,
                        } => {
                            if status == StatusCode::NOT_FOUND {
                                eprintln!("Could not find the media asset: #{media_id}");
                            } else {
                                print_request_error(e);
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

    let tags_vec = tags_res.unwrap();

    let tags_map = tags_map_from_list(&tags_vec);

    // Tag

    let tag_param: String;

    if is_identifier(&tag) {
        let tag_id_res = parse_identifier(&tag);

        match tag_id_res {
            Ok(tag_id) => {
                if tags_map.contains_key(&tag_id) {
                    tag_param = (*tags_map.get(&tag_id).unwrap()).clone();
                } else {
                    tag_param = tag;
                }
            }
            Err(_) => {
                tag_param = tag;
            }
        }
    } else {
        tag_param = tag;
    }

    // Call API

    let api_res = api_call_tag_add(
        vault_url.clone(),
        AddTagBody {
            media_id: media_id_param,
            tag_name: tag_param,
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(added_tag) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let added_tag_id = added_tag.id;
            let added_tag_name = added_tag.name;

            eprintln!("Successfully added tag #{added_tag_id} ( {added_tag_name} ) to media asset #{media_id_param}");
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

pub async fn run_cmd_tag_remove(
    global_opts: CommandGlobalOptions,
    tag: String,
    media: String,
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
            media_id_param = media_id;
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

    let tags_vec = tags_res.unwrap();

    let tags_map = tags_map_from_list(&tags_vec);
    let tags_reverse_map = tags_reverse_map_from_list(&tags_vec);

    // Tag

    let tag_param: u64;

    if is_identifier(&tag) {
        let tag_id_res = parse_identifier(&tag);

        match tag_id_res {
            Ok(tag_id) => {
                tag_param = tag_id;
            }
            Err(_) => {
                let parsed_tag = parse_tag_name(&tag);
                if tags_reverse_map.contains_key(&parsed_tag) {
                    tag_param = *tags_reverse_map.get(&parsed_tag).unwrap();
                } else {
                    if logout_after_operation {
                        let logout_res = do_logout(global_opts, vault_url.clone()).await;

                        match logout_res {
                            Ok(_) => {}
                            Err(_) => {
                                process::exit(1);
                            }
                        }
                    }

                    eprintln!("Tag not found: {parsed_tag}");
                    process::exit(1);
                }
            }
        }
    } else {
        let parsed_tag = parse_tag_name(&tag);
        if tags_reverse_map.contains_key(&parsed_tag) {
            tag_param = *tags_reverse_map.get(&parsed_tag).unwrap();
        } else {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            eprintln!("Tag not found: {parsed_tag}");
            process::exit(1);
        }
    }

    let default_tag_name = "???".to_string();
    let tag_name = tags_map.get(&tag_param).unwrap_or(&default_tag_name);

    // Call API

    let api_res = api_call_tag_remove(
        vault_url.clone(),
        RemoveTagBody {
            media_id: media_id_param,
            tag_id: tag_param,
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

            eprintln!("Successfully removed tag #{tag_param} ( {tag_name} ) from media asset #{media_id_param}");
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
