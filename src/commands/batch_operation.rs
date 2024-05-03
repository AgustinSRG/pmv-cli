// Batch operation command

use std::process;

use clap::Subcommand;

use crate::{
    api::{
        api_call_album_add_media, api_call_album_remove_media, api_call_get_album, api_call_get_tags, api_call_media_delete, api_call_search_advanced, api_call_tag_add, api_call_tag_remove, MAX_API_TAGS_FILTER, MAX_SEARCH_PAGE_LIMIT
    },
    models::{
        parse_media_type, parse_tag_name, parse_tag_search_mode, tags_map_from_list,
        tags_reverse_map_from_list, AddTagBody, AlbumMediaBody, MediaListItem, MediaType,
        RemoveTagBody, TagSearchMode,
    },
    tools::{
        ask_user, ensure_login, identifier_to_string, parse_identifier, parse_vault_uri,
        to_csv_string, VaultURI,
    },
};

use super::{
    get_vault_url, logout::do_logout, print_request_error, search_advanced::media_matches_filter,
    CommandGlobalOptions,
};

#[derive(Subcommand)]
pub enum BatchCommand {
    /// Adds tags to the media assets
    AddTags {
        /// List of tag names, separated by spaces.
        tags: String,
    },

    /// Removes tags from the media assets
    RemoveTags {
        /// List of tag names, separated by spaces.
        tags: String,
    },

    /// Adds media assets into an album
    AddToAlbum {
        /// Album ID
        album: String,
    },

    /// Removes media assets from an album, if they were in it
    RemoveFromAlbum {
        /// Album ID
        album: String,
    },

    /// Delete media assets
    Delete,
}

#[allow(clippy::too_many_arguments)]
pub async fn run_cmd_batch_operation(
    global_opts: CommandGlobalOptions,
    title: Option<String>,
    description: Option<String>,
    media_type: Option<String>,
    tags: Option<String>,
    tags_mode: Option<String>,
    album: Option<String>,
    everything: bool,
    batch_command: BatchCommand,
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

    let tags_reverse_map = tags_reverse_map_from_list(&tags_vec);

    // Get album (if necessary)

    let mut album_filter: Option<Vec<MediaListItem>> = None;

    if let Some(album_id_str) = album {
        let album_id_res = parse_identifier(&album_id_str);

        match album_id_res {
            Ok(_) => {
                let album_get_api_res =
                    api_call_get_album(&vault_url, album_id_res.unwrap(), global_opts.debug)
                        .await;

                match album_get_api_res {
                    Ok(album_data) => {
                        album_filter = Some(album_data.list);
                    }
                    Err(e) => {
                        print_request_error(e);
                        process::exit(1);
                    }
                }
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
                eprintln!("Invalid album identifier specified for the album filtering option.");
                process::exit(1);
            }
        }
    }

    // Params

    let title_filter = title.unwrap_or("".to_string());
    let description_filter = description.unwrap_or("".to_string());

    let mut media_type_filter: Option<MediaType> = None;

    if let Some(media_type_str) = media_type {
        let media_type_res = parse_media_type(&media_type_str);

        match media_type_res {
            Ok(m) => {
                media_type_filter = Some(m);
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
                eprintln!("Media type not recognized: {media_type_str} | Valid ones are: video, image or audio");
                process::exit(1);
            }
        }
    }

    let mut tags_filter: Option<Vec<u64>> = None;
    let mut tag_param: Option<Vec<String>> = None;
    let mut tags_filter_count: usize = 0;

    if let Some(tags_str) = tags {
        let tag_names = tags_str.split(' ');

        let mut tag_ids: Vec<u64> = Vec::new();
        let mut tag_names_param: Vec<String> = Vec::new();

        for tag_name in tag_names {
            let parsed_tag_name = parse_tag_name(tag_name);
            if parsed_tag_name.is_empty() {
                continue;
            }

            if !tags_reverse_map.contains_key(&parsed_tag_name) {
                if logout_after_operation {
                    let logout_res = do_logout(&global_opts, &vault_url).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }
                eprintln!("Could not find tag with name: {tag_name}");
                process::exit(1);
            }

            if tag_names_param.len() < MAX_API_TAGS_FILTER {
                tag_names_param.push(parsed_tag_name.clone());
            }

            tag_ids.push(*tags_reverse_map.get(&parsed_tag_name).unwrap());

            tags_filter_count += 1;
        }

        tags_filter = Some(tag_ids);
        tag_param = Some(tag_names_param);
    }

    let mut tags_filter_mode = TagSearchMode::All;

    if let Some(tags_mode_str) = tags_mode{
        let tags_mode_res = parse_tag_search_mode(&tags_mode_str);

        match tags_mode_res {
            Ok(m) => {
                tags_filter_mode = m;
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
                eprintln!("Tags filtering mode not recognized: {tags_mode_str} | Valid ones are: all, any, none or untagged");
                process::exit(1);
            }
        }
    }

    let tag_mode_api_param: String = match tags_filter_mode {
        TagSearchMode::All => "allof".to_string(),
        TagSearchMode::Any => {
            if tags_filter_count > MAX_API_TAGS_FILTER {
                "allof".to_string()
            } else {
                "anyof".to_string()
            }
        },
        TagSearchMode::None => {
            "noneof".to_string()
        },
        TagSearchMode::Untagged => "allof".to_string(),
    };

    match tags_filter_mode {
        TagSearchMode::All => {},
        TagSearchMode::Any => {
            if tags_filter_count > MAX_API_TAGS_FILTER {
                tag_param = None
            }
        },
        TagSearchMode::None => {},
        TagSearchMode::Untagged => {
            tag_param = None
        },
    }

    if everything {
        if tags_filter_mode != TagSearchMode::All
            || tag_param.is_some()
            || media_type_filter.is_some()
            || !description_filter.is_empty()
            || !title_filter.is_empty()
            || album_filter.is_some()
        {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Error: The --everything option is incompatible with any other filter.");
            process::exit(1);
        }
    } else if tags_filter_mode == TagSearchMode::All
        && tag_param.is_none()
        && media_type_filter.is_none()
        && description_filter.is_empty()
        && title_filter.is_empty()
        && album_filter.is_none()
    {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: You must specify at least one filter. Use the --everything option if you want to apply the operation to the entire vault.");
        process::exit(1);
    }

    // Search results and push them into a list

    let mut advanced_search_results: Vec<MediaListItem> = Vec::new();

    match album_filter {
        Some(album_list) => {
            for item in album_list {
                if media_matches_filter(
                    &item,
                    &title_filter,
                    &description_filter,
                    &media_type_filter,
                    &tags_filter,
                    &tags_filter_mode,
                ) {
                    advanced_search_results.push(item);
                }
            }
        }
        None => {
            let mut advanced_search_finished = false;
            let mut continue_ref: Option<u64> = None;
            while !advanced_search_finished {
                // Call API
                let api_res = api_call_search_advanced(
                    &vault_url,
                    tag_param.as_deref(),
                    &tag_mode_api_param,
                    false,
                    MAX_SEARCH_PAGE_LIMIT as u32,
                    continue_ref,
                    global_opts.debug,
                )
                .await;

                match api_res {
                    Ok(search_result) => {
                        for item in search_result.page_items {
                            if media_matches_filter(
                                &item,
                                &title_filter,
                                &description_filter,
                                &media_type_filter,
                                &tags_filter,
                                &tags_filter_mode,
                            ) {
                                advanced_search_results.push(item);
                            }
                        }

                        if search_result.scanned >= search_result.total_count {
                            advanced_search_finished = true;
                        } 

                        continue_ref = Some(search_result.continue_ref);
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
            }
        }
    }

    apply_batch_operation(
        global_opts,
        vault_url,
        logout_after_operation,
        advanced_search_results,
        batch_command,
    )
    .await;
}

async fn apply_batch_operation(
    global_opts: CommandGlobalOptions,
    vault_url: VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
    batch_command: BatchCommand,
) {
    if media_list.is_empty() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: Could not find any media assets with the specified filter.");
        process::exit(1);
    }

    match batch_command {
        BatchCommand::AddTags { tags } => {
            batch_add_tags(
                global_opts.clone(),
                &vault_url,
                logout_after_operation,
                media_list,
                tags,
            )
            .await;
        }
        BatchCommand::RemoveTags { tags } => {
            batch_remove_tags(
                global_opts.clone(),
                &vault_url,
                logout_after_operation,
                media_list,
                tags,
            )
            .await;
        }
        BatchCommand::AddToAlbum { album } => {
            batch_add_to_album(
                global_opts.clone(),
                &vault_url,
                logout_after_operation,
                media_list,
                album,
            )
            .await;
        }
        BatchCommand::RemoveFromAlbum { album } => {
            batch_remove_from_album(
                global_opts.clone(),
                &vault_url,
                logout_after_operation,
                media_list,
                album,
            )
            .await;
        }
        BatchCommand::Delete => {
            batch_delete(
                global_opts.clone(),
                &vault_url,
                logout_after_operation,
                media_list,
            )
            .await;
        }
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

    eprint!("Done!");
}

async fn batch_add_tags(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
    tags: String,
) {
    let n_total = media_list.len();

    let mut tags_to_add: Vec<String> = Vec::new();

    let tag_names = tags.split(' ');

    for tag_name in tag_names {
        let parsed_tag_name = parse_tag_name(tag_name);
        if parsed_tag_name.is_empty() {
            continue;
        }

        tags_to_add.push(parsed_tag_name);
    }

    if tags_to_add.is_empty() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: Tag list is empty.");
        process::exit(1);
    }

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to add the specified tags to {n_total} media assets?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Operation loop

    let mut n_done: usize = 0;

    for media in media_list {
        n_done += 1;
        for tag in tags_to_add.iter() {
            let api_res = api_call_tag_add(
                vault_url,
                AddTagBody {
                    media_id: media.id,
                    tag_name: tag.to_string(),
                },
                global_opts.debug,
            )
            .await;

            match api_res {
                Ok(_) => {
                    let media_id_str = identifier_to_string(media.id);
                    let tag_str = to_csv_string(tag);
                    let media_title = to_csv_string(&media.title);
                    eprintln!("[{n_done}/{n_total}] Added tag {tag_str} to media {media_id_str}: {media_title}");
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
    }
}

async fn batch_remove_tags(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
    tags: String,
) {
    let n_total = media_list.len();

    // Get tags

    let tags_res = api_call_get_tags(vault_url, global_opts.debug).await;

    if tags_res.is_err() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, vault_url).await;

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

    let mut tags_to_remove: Vec<u64> = Vec::new();

    let tag_names = tags.split(' ');

    for tag_name in tag_names {
        let parsed_tag_name = parse_tag_name(tag_name);
        if parsed_tag_name.is_empty() {
            continue;
        }

        if !tags_reverse_map.contains_key(&parsed_tag_name) {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            eprintln!("Could not find tag with name: {tag_name}");
            process::exit(1);
        }

        tags_to_remove.push(*tags_reverse_map.get(&parsed_tag_name).unwrap());
    }

    if tags_to_remove.is_empty() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: Tag list is empty.");
        process::exit(1);
    }

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!(
            "Are you sure you want to remove the specified tags from {n_total} media assets?"
        );
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Operation loop

    let mut n_done: usize = 0;

    let default_tag_name = "???".to_string();
    for media in media_list {
        n_done += 1;

        for tag in tags_to_remove.iter() {
            let tag_name = tags_map.get(tag).unwrap_or(&default_tag_name);

            let api_res = api_call_tag_remove(
                vault_url,
                RemoveTagBody {
                    media_id: media.id,
                    tag_id: *tag,
                },
                global_opts.debug,
            )
            .await;

            match api_res {
                Ok(_) => {
                    let media_id_str = identifier_to_string(media.id);
                    let tag_str = to_csv_string(tag_name);
                    let media_title = to_csv_string(&media.title);
                    eprintln!("[{n_done}/{n_total}] Removed tag {tag_str} from media {media_id_str}: {media_title}");
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
    }
}

async fn batch_add_to_album(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
    album: String,
) {
    let n_total = media_list.len();

    let album_id_res = parse_identifier(&album);
    let album_id: u64;
    let album_id_str: String;

    match album_id_res {
        Ok(id) => {
            album_id = id;
            album_id_str = identifier_to_string(id);
        }
        Err(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, vault_url).await;

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

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to add {n_total} media assets to album {album_id_str}?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Operation loop

    let mut n_done: usize = 0;

    for media in media_list {
        n_done += 1;

        let api_res = api_call_album_add_media(
            vault_url,
            album_id,
            AlbumMediaBody { media_id: media.id },
            global_opts.debug,
        )
        .await;

        match api_res {
            Ok(_) => {
                let media_id_str = identifier_to_string(media.id);
                let media_title = to_csv_string(&media.title);
                eprintln!("[{n_done}/{n_total}] Added media {media_id_str}: {media_title} into album {album_id_str}");
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
}

async fn batch_remove_from_album(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
    album: String,
) {
    let n_total = media_list.len();

    let album_id_res = parse_identifier(&album);
    let album_id: u64;
    let album_id_str: String;

    match album_id_res {
        Ok(id) => {
            album_id = id;
            album_id_str = identifier_to_string(id);
        }
        Err(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, vault_url).await;

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

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!(
            "Are you sure you want to remove {n_total} media assets from album {album_id_str}?"
        );
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Operation loop

    let mut n_done: usize = 0;

    for media in media_list {
        n_done += 1;

        let api_res = api_call_album_remove_media(
            vault_url,
            album_id,
            AlbumMediaBody { media_id: media.id },
            global_opts.debug,
        )
        .await;

        match api_res {
            Ok(_) => {
                let media_id_str = identifier_to_string(media.id);
                let media_title = to_csv_string(&media.title);
                eprintln!("[{n_done}/{n_total}] Removed media {media_id_str}: {media_title} from album {album_id_str}");
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
}

async fn batch_delete(
    global_opts: CommandGlobalOptions,
    vault_url: &VaultURI,
    logout_after_operation: bool,
    media_list: Vec<MediaListItem>,
) {
    let n_total = media_list.len();

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to delete {n_total} media assets from the vault?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Operation loop

    let mut n_done: usize = 0;

    for media in media_list {
        n_done += 1;

        let api_res = api_call_media_delete(vault_url, media.id, global_opts.debug).await;

        match api_res {
            Ok(_) => {
                let media_id_str = identifier_to_string(media.id);
                let media_title = to_csv_string(&media.title);
                eprintln!("[{n_done}/{n_total}] Deleted media {media_id_str}: {media_title}");
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
}
