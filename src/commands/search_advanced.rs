// Advanced search

use std::process;

use crate::api::{api_call_get_album, api_call_search_advanced, MAX_API_TAGS_FILTER};
use crate::models::{
    parse_media_type, parse_tag_name, parse_tag_search_mode, tags_reverse_map_from_list,
    MediaListItem, MediaType, TagSearchMode,
};
use crate::tools::{render_media_duration, to_csv_string};
use crate::{
    api::api_call_get_tags,
    models::{tags_map_from_list, tags_names_from_ids},
    tools::{ensure_login, identifier_to_string, parse_identifier, parse_vault_uri, print_table},
};

use super::{get_vault_url, logout::do_logout, print_request_error, CommandGlobalOptions};

const DEFAULT_RESULTS_LIMIT: u32 = 25;

#[allow(clippy::too_many_arguments)]
pub async fn run_cmd_search_advanced(
    global_opts: CommandGlobalOptions,
    title: Option<String>,
    description: Option<String>,
    media_type: Option<String>,
    tags: Option<String>,
    tags_mode: Option<String>,
    album: Option<String>,
    limit: Option<u32>,
    start_from: Option<String>,
    reverse: bool,
    extended: bool,
    csv: bool,
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

    let tags_map = tags_map_from_list(&tags_vec);
    let tags_reverse_map = tags_reverse_map_from_list(&tags_vec);

    // Get album (if necessary)

    let mut album_filter: Option<Vec<MediaListItem>> = None;

    if let Some(album_id_str) = album {
        let album_id_res = parse_identifier(&album_id_str);

        match album_id_res {
            Ok(_) => {
                let album_get_api_res =
                    api_call_get_album(&vault_url, album_id_res.unwrap(), global_opts.debug).await;

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

    let mut start_from_param: Option<u64> = None;

    if let Some(start_from_str) = start_from {
        let start_from_res = parse_identifier(&start_from_str);

        match start_from_res {
            Ok(start_from_u64) => {
                start_from_param = Some(start_from_u64);
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
                eprintln!("Invalid media identifier specified for the start-from option.");
                process::exit(1);
            }
        }
    }

    let limit_param = limit.unwrap_or(DEFAULT_RESULTS_LIMIT);

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

        let mut tag_names_param: Vec<String> = Vec::new();
        let mut tag_ids: Vec<u64> = Vec::new();

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

    if let Some(tags_mode_str) = tags_mode {
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
        }
        TagSearchMode::None => "noneof".to_string(),
        TagSearchMode::Untagged => "allof".to_string(),
    };

    match tags_filter_mode {
        TagSearchMode::All => {}
        TagSearchMode::Any => {
            if tags_filter_count > MAX_API_TAGS_FILTER {
                tag_param = None
            }
        }
        TagSearchMode::None => {}
        TagSearchMode::Untagged => tag_param = None,
    }

    // Search

    let mut advanced_search_results: Vec<MediaListItem> = Vec::new();

    match album_filter {
        Some(album_list) => {
            if reverse {
                for item in album_list.iter().rev() {
                    if media_matches_filter(
                        item,
                        &title_filter,
                        &description_filter,
                        &media_type_filter,
                        &tags_filter,
                        &tags_filter_mode,
                    ) {
                        advanced_search_results.push(item.clone());

                        if advanced_search_results.len() as u32 >= limit_param {
                            break;
                        }
                    }
                }
            } else {
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

                        if advanced_search_results.len() as u32 >= limit_param {
                            break;
                        }
                    }
                }
            }
        }
        None => {
            // Search loop
            let mut advanced_search_finished = false;
            let mut continue_ref: Option<u64> = start_from_param;
            while !advanced_search_finished {
                if advanced_search_results.len() as u32 >= limit_param {
                    break;
                }

                // Call API
                let api_res = api_call_search_advanced(
                    &vault_url,
                    tag_param.as_deref(),
                    &tag_mode_api_param,
                    reverse,
                    limit_param,
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

                                if advanced_search_results.len() as u32 >= limit_param {
                                    advanced_search_finished = true;
                                    break;
                                }
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

    // Print results

    if logout_after_operation {
        let logout_res = do_logout(&global_opts, &vault_url).await;

        match logout_res {
            Ok(_) => {}
            Err(_) => {
                process::exit(1);
            }
        }
    }

    let items_count = advanced_search_results.len();

    println!("items retrieved: {items_count}");

    if csv {
        println!();
        if !extended {
            println!("\"Id\",\"Type\",\"Title\"");

            for item in advanced_search_results {
                let row_id = item.id.to_string();
                let row_type = to_csv_string(&item.media_type.to_type_string());
                let row_title = to_csv_string(&item.title);
                println!("{row_id},{row_type},{row_title}");
            }
        } else {
            println!("\"Id\",\"Type\",\"Title\",\"Description\",\"Tags\",\"Duration\"");

            for item in advanced_search_results {
                let row_id = item.id.to_string();
                let row_type = to_csv_string(&item.media_type.to_type_string());
                let row_title = to_csv_string(&item.title);
                let row_description = to_csv_string(&item.description);
                let row_tags = to_csv_string(&tags_names_from_ids(&item.tags, &tags_map).join(" "));
                let row_duration =
                    render_media_duration(item.media_type, item.duration.unwrap_or(0.0));

                println!(
                    "{row_id},{row_type},{row_title},{row_description},{row_tags},{row_duration}"
                );
            }
        }
    } else if !extended {
        let table_head: Vec<String> =
            vec!["Id".to_string(), "Type".to_string(), "Title".to_string()];
        let mut table_body: Vec<Vec<String>> = Vec::with_capacity(items_count);

        for item in advanced_search_results {
            table_body.push(vec![
                identifier_to_string(item.id).clone(),
                item.media_type.to_type_string(),
                to_csv_string(&item.title),
            ]);
        }

        print_table(&table_head, &table_body, false);
    } else {
        let table_head: Vec<String> = vec![
            "Id".to_string(),
            "Type".to_string(),
            "Title".to_string(),
            "Description".to_string(),
            "Tags".to_string(),
            "Duration".to_string(),
        ];
        let mut table_body: Vec<Vec<String>> = Vec::with_capacity(items_count);

        for item in advanced_search_results {
            table_body.push(vec![
                identifier_to_string(item.id).clone(),
                item.media_type.to_type_string(),
                to_csv_string(&item.title),
                to_csv_string(&item.description),
                to_csv_string(&tags_names_from_ids(&item.tags, &tags_map).join(" ")),
                render_media_duration(item.media_type, item.duration.unwrap_or(0.0)),
            ]);
        }

        print_table(&table_head, &table_body, false);
    }
}

pub fn media_matches_filter(
    media: &MediaListItem,
    title: &str,
    description: &str,
    media_type_filter: &Option<MediaType>,
    tags_filter: &Option<Vec<u64>>,
    tags_filter_mode: &TagSearchMode,
) -> bool {
    if !title.is_empty() && !media.title.contains(title) {
        return false;
    }

    if !description.is_empty() && !media.description.contains(description) {
        return false;
    }

    if let Some(t) = media_type_filter {
        if *t != media.media_type {
            return false;
        }
    }

    match tags_filter_mode {
        TagSearchMode::All => {
            if let Some(tags) = tags_filter {
                for tag_m in tags {
                    if !media.tags.contains(tag_m) {
                        return false;
                    }
                }
            }
        }
        TagSearchMode::Any => {
            if let Some(tags) = tags_filter {
                let mut has_any = false;
                for tag_m in tags {
                    if media.tags.contains(tag_m) {
                        has_any = true;
                        break;
                    }
                }

                if !has_any {
                    return false;
                }
            }
        }
        TagSearchMode::None => {
            if let Some(tags) = tags_filter {
                for tag_m in &media.tags {
                    if tags.contains(tag_m) {
                        return false;
                    }
                }
            }
        }
        TagSearchMode::Untagged => {
            if !media.tags.is_empty() {
                return false;
            }
        }
    }

    true
}
