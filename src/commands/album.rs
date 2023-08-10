// Album command

use std::{collections::HashSet, process};

use clap::Subcommand;
use hyper::StatusCode;

use crate::{
    api::{
        api_call_album_add_media, api_call_album_remove_media, api_call_album_set_order,
        api_call_create_album, api_call_delete_album, api_call_get_album, api_call_get_albums,
        api_call_get_media, api_call_get_media_albums, api_call_get_tags, api_call_rename_album,
    },
    commands::logout::do_logout,
    models::{
        tags_map_from_list, tags_names_from_ids, AlbumListItem, AlbumMediaBody, AlbumNameBody,
        AlbumSetOrderBody,
    },
    tools::{
        ask_user, ensure_login, format_date, identifier_to_string, parse_identifier,
        parse_vault_uri, print_table, render_media_duration, to_csv_string,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum AlbumCommand {
    /// List albums
    #[clap(alias("ls"))]
    List {
        /// Filter by media
        #[arg(short, long)]
        media: Option<String>,

        /// CSV format
        #[arg(short, long)]
        csv: bool,

        /// Sort alphabetically by name
        #[arg(short, long)]
        alphabetically: bool,

        /// Sort by ID
        #[arg(short, long)]
        id_sorted: bool,
    },

    /// Get album and prints it
    Get {
        /// Album ID
        album: String,

        /// Extended version of the results table
        #[arg(short, long)]
        extended: bool,

        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Creates a new album
    Create {
        /// Album name
        name: String,
    },

    /// Renames an album
    Rename {
        /// Album ID
        album: String,

        /// Album name
        name: String,
    },

    /// Deletes album
    Delete {
        /// Album ID
        album: String,
    },

    /// Adds a media asset to an album
    Add {
        /// Album ID
        album: String,

        /// Media asset ID
        media: String,
    },

    /// Removes a media asset from an album
    Remove {
        /// Album ID
        album: String,

        /// Media asset ID
        media: String,
    },

    /// Changes the position of a media asset inside al album
    SetPosition {
        /// Album ID
        album: String,

        /// Media asset ID
        media: String,

        /// New position for the media asset, starting at 1
        position: u32,
    },
}

pub async fn run_album_cmd(global_opts: CommandGlobalOptions, cmd: AlbumCommand) {
    match cmd {
        AlbumCommand::List {
            media,
            csv,
            alphabetically,
            id_sorted,
        } => {
            run_cmd_list_albums(global_opts, media, csv, alphabetically, id_sorted).await;
        }
        AlbumCommand::Get {
            album,
            extended,
            csv,
        } => {
            run_cmd_get_album(global_opts, album, csv, extended).await;
        }
        AlbumCommand::Create { name } => {
            run_cmd_album_create(global_opts, name).await;
        }
        AlbumCommand::Rename { album, name } => {
            run_cmd_album_rename(global_opts, album, name).await;
        }
        AlbumCommand::Delete { album } => {
            run_cmd_album_delete(global_opts, album).await;
        }
        AlbumCommand::Add { album, media } => {
            run_cmd_album_add_media(global_opts, album, media).await;
        }
        AlbumCommand::Remove { album, media } => {
            run_cmd_album_remove_media(global_opts, album, media).await;
        }
        AlbumCommand::SetPosition {
            album,
            media,
            position,
        } => {
            run_cmd_album_media_change_position(global_opts, album, media, position).await;
        }
    }
}

pub async fn run_cmd_list_albums(
    global_opts: CommandGlobalOptions,
    media: Option<String>,
    csv: bool,
    alphabetically: bool,
    id_sorted: bool,
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

    // Get media albums

    let mut albums_media_filter: Option<HashSet<u64>> = None;

    if media.is_some() {
        let media_id_res = parse_identifier(&media.unwrap());

        match media_id_res {
            Ok(media_id) => {
                let api_media_albums_res =
                    api_call_get_media_albums(vault_url.clone(), media_id, global_opts.debug).await;

                match api_media_albums_res {
                    Ok(list) => {
                        let mut list_set: HashSet<u64> = HashSet::new();
                        for a_id in list {
                            list_set.insert(a_id);
                        }
                        albums_media_filter = Some(list_set);
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
    }

    // Call API

    let api_res = api_call_get_albums(vault_url.clone(), global_opts.debug).await;

    match api_res {
        Ok(original_albums_list) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            // Filter

            let mut albums: Vec<AlbumListItem>;

            match albums_media_filter {
                Some(filter_list) => {
                    albums = original_albums_list
                        .into_iter()
                        .filter(|a| filter_list.contains(&a.id))
                        .collect();
                }
                None => {
                    albums = original_albums_list;
                }
            }

            // Sort

            if alphabetically {
                albums.sort_by(|a, b| a.name.cmp(&b.name));
            } else if id_sorted {
                albums.sort_by(|a, b| a.id.cmp(&b.id));
            } else {
                albums.sort_by(|a, b| b.lm.cmp(&a.lm));
            }

            let total = albums.len();

            println!("total: {total}");

            if csv {
                println!();
                println!("\"Album Id\",\"Album Name\",\"Size\",\"Last Modified\"");

                for album in albums {
                    let row_id = identifier_to_string(album.id);
                    let row_name = to_csv_string(&album.name);
                    let row_size = album.size.to_string();
                    let row_lm = format_date(album.lm);
                    println!("{row_id},{row_name},{row_size},{row_lm}");
                }
            } else {
                let table_head: Vec<String> = vec![
                    "Album Id".to_string(),
                    "Album Name".to_string(),
                    "Size".to_string(),
                    "Last Modified".to_string(),
                ];

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(total);

                for album in albums {
                    table_body.push(vec![
                        identifier_to_string(album.id),
                        to_csv_string(&album.name),
                        album.size.to_string(),
                        format_date(album.lm),
                    ]);
                }

                print_table(&table_head, &table_body, false);
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

pub async fn run_cmd_get_album(
    global_opts: CommandGlobalOptions,
    album: String,
    csv: bool,
    extended: bool,
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

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

    let api_res = api_call_get_album(vault_url.clone(), album_id, global_opts.debug).await;

    match api_res {
        Ok(album_data) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let album_name = album_data.name;
            let album_lm = format_date(album_data.lm);
            let album_size = album_data.list.len();

            println!("name: {album_name}");
            println!("last modified: {album_lm}");
            println!("size: {album_size}");

            if csv {
                println!();
                if !extended {
                    println!("\"Pos\",\"Id\",\"Type\",\"Title\"");

                    for (i, item) in album_data.list.iter().enumerate() {
                        let row_pos = i + 1;
                        let row_id = item.id.to_string();
                        let row_type = to_csv_string(&item.media_type.to_type_string());
                        let row_title = to_csv_string(&item.title);
                        println!("{row_pos},{row_id},{row_type},{row_title}");
                    }
                } else {
                    println!(
                        "\"Pos\",\"Id\",\"Type\",\"Title\",\"Description\",\"Tags\",\"Duration\""
                    );

                    for (i, item) in album_data.list.iter().enumerate() {
                        let row_pos = i + 1;
                        let row_id = item.id.to_string();
                        let row_type = to_csv_string(&item.media_type.to_type_string());
                        let row_title = to_csv_string(&item.title);
                        let row_description = to_csv_string(&item.description);
                        let row_tags =
                            to_csv_string(&tags_names_from_ids(&item.tags, &tags).join(" "));
                        let row_duration =
                            render_media_duration(item.media_type, item.duration.unwrap_or(0.0));

                        println!("{row_pos},{row_id},{row_type},{row_title},{row_description},{row_tags},{row_duration}");
                    }
                }
            } else if !extended {
                let table_head: Vec<String> = vec![
                    "Pos".to_string(),
                    "Id".to_string(),
                    "Type".to_string(),
                    "Title".to_string(),
                ];
                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(album_size);

                for (i, item) in album_data.list.iter().enumerate() {
                    table_body.push(vec![
                        (i + 1).to_string(),
                        identifier_to_string(item.id).clone(),
                        item.media_type.to_type_string(),
                        to_csv_string(&item.title),
                    ]);
                }

                print_table(&table_head, &table_body, false);
            } else {
                let table_head: Vec<String> = vec![
                    "Pos".to_string(),
                    "Id".to_string(),
                    "Type".to_string(),
                    "Title".to_string(),
                    "Description".to_string(),
                    "Tags".to_string(),
                    "Duration".to_string(),
                ];
                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(album_size);

                for (i, item) in album_data.list.iter().enumerate() {
                    table_body.push(vec![
                        (i + 1).to_string(),
                        identifier_to_string(item.id).clone(),
                        item.media_type.to_type_string(),
                        to_csv_string(&item.title),
                        to_csv_string(&item.description),
                        to_csv_string(&tags_names_from_ids(&item.tags, &tags).join(" ")),
                        render_media_duration(item.media_type, item.duration.unwrap_or(0.0)),
                    ]);
                }

                print_table(&table_head, &table_body, false);
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

pub async fn run_cmd_album_create(global_opts: CommandGlobalOptions, name: String) {
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

    let api_res = api_call_create_album(
        vault_url.clone(),
        AlbumNameBody { name: name.clone() },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(added_album) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let added_album_id = added_album.album_id;

            eprintln!("Successfully created album #{added_album_id}: {name}");
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

pub async fn run_cmd_album_rename(global_opts: CommandGlobalOptions, album: String, name: String) {
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

    // Call API

    let api_res = api_call_rename_album(
        vault_url.clone(),
        album_id,
        AlbumNameBody { name: name.clone() },
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

            eprintln!("Successfully renamed album #{album_id}: {name}");
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

pub async fn run_cmd_album_delete(global_opts: CommandGlobalOptions, album: String) {
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

    // Get album

    let api_get_res = api_call_get_album(vault_url.clone(), album_id, global_opts.debug).await;

    let album_name: String = match api_get_res {
        Ok(album_data) => album_data.name,
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
    };

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to delete the album {album_name}?");
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

    let api_res = api_call_delete_album(vault_url.clone(), album_id, global_opts.debug).await;

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

            eprintln!("Successfully deleted album #{album_id}: {album_name}");
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

pub async fn run_cmd_album_add_media(
    global_opts: CommandGlobalOptions,
    album: String,
    media: String,
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

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
                        crate::tools::RequestError::StatusCode(_)
                        | crate::tools::RequestError::Hyper(_)
                        | crate::tools::RequestError::FileSystem(_)
                        | crate::tools::RequestError::Json {
                            message: _,
                            body: _,
                        } => {
                            print_request_error(e);
                        }
                        crate::tools::RequestError::Api {
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

    // Get album

    let api_get_res = api_call_get_album(vault_url.clone(), album_id, global_opts.debug).await;
    let album_name: String;

    match api_get_res {
        Ok(album_data) => {
            album_name = album_data.name;

            // Check if the media is in the album
            let mut media_is_in_album = false;

            for m in album_data.list {
                if m.id == media_id_param {
                    media_is_in_album = true;
                    break;
                }
            }

            if media_is_in_album {
                if logout_after_operation {
                    let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }
                eprintln!("Media asset #{media_id_param} is already inside the album #{album_id}: {album_name}");
                process::exit(1);
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

    // Call API

    let api_res = api_call_album_add_media(
        vault_url.clone(),
        album_id,
        AlbumMediaBody {
            media_id: media_id_param,
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

            eprintln!("Successfully added media asset #{media_id_param} to album #{album_id}: {album_name}");
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

pub async fn run_cmd_album_remove_media(
    global_opts: CommandGlobalOptions,
    album: String,
    media: String,
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

    // Media ID

    let media_id_res = parse_identifier(&media);

    let media_id_param: u64 = match media_id_res {
        Ok(media_id) => media_id,
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
    };

    // Get album

    let api_get_res = api_call_get_album(vault_url.clone(), album_id, global_opts.debug).await;
    let album_name: String;

    match api_get_res {
        Ok(album_data) => {
            album_name = album_data.name;

            // Check if the media is in the album
            let mut media_is_in_album = false;

            for m in album_data.list {
                if m.id == media_id_param {
                    media_is_in_album = true;
                    break;
                }
            }

            if !media_is_in_album {
                if logout_after_operation {
                    let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }
                eprintln!("Media asset #{media_id_param} is not inside the album #{album_id}: {album_name}");
                process::exit(1);
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

    // Call API

    let api_res = api_call_album_remove_media(
        vault_url.clone(),
        album_id,
        AlbumMediaBody {
            media_id: media_id_param,
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

            eprintln!("Successfully removed media asset #{media_id_param} from album #{album_id}: {album_name}");
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

pub async fn run_cmd_album_media_change_position(
    global_opts: CommandGlobalOptions,
    album: String,
    media: String,
    position: u32,
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

    let album_id_res = parse_identifier(&album);
    let album_id: u64 = match album_id_res {
        Ok(id) => id,
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
    };

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
                        crate::tools::RequestError::StatusCode(_)
                        | crate::tools::RequestError::Hyper(_)
                        | crate::tools::RequestError::FileSystem(_)
                        | crate::tools::RequestError::Json {
                            message: _,
                            body: _,
                        } => {
                            print_request_error(e);
                        }
                        crate::tools::RequestError::Api {
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

    // Get album

    let api_get_res = api_call_get_album(vault_url.clone(), album_id, global_opts.debug).await;
    let album_name: String;

    let mut album_new_list: Vec<u64> = Vec::new();
    let mut actual_inserted_position = position;

    match api_get_res {
        Ok(album_data) => {
            album_name = album_data.name;

            // Remove media asset from the album, if present
            let filtered_list: Vec<u64> = album_data
                .list
                .into_iter()
                .map(|m| m.id)
                .filter(|id| *id != media_id_param)
                .collect();

            // Insert
            let mut inserted = false;

            if position < 1 {
                album_new_list.push(media_id_param);
                inserted = true;
                actual_inserted_position = 1;
            }

            for (i, m_id) in filtered_list.into_iter().enumerate() {
                if position == (i + 1) as u32 {
                    album_new_list.push(media_id_param);
                    inserted = true;
                    actual_inserted_position = (1 + 1) as u32;
                }

                album_new_list.push(m_id);
            }

            if !inserted {
                album_new_list.push(media_id_param);
                actual_inserted_position = album_new_list.len() as u32;
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

    // Call API

    let api_res = api_call_album_set_order(
        vault_url.clone(),
        album_id,
        AlbumSetOrderBody {
            list: album_new_list,
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

            eprintln!("Successfully inserted media asset #{media_id_param} into position {actual_inserted_position} of album #{album_id}: {album_name}");
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
