// Random media list command

use std::{process, iter, time::Duration};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::tools::render_media_duration;
use crate::{tools::{ensure_login, parse_vault_uri, print_table, is_identifier, parse_identifier, identifier_to_string}, api::{api_call_random, api_call_get_tags}, models::{tags_map_from_list, tags_names_from_ids}};

use super::{CommandGlobalOptions, logout::do_logout, get_vault_url, print_request_error};

const DEFAULT_PAGE_SIZE: u32 = 10;

pub async fn run_cmd_random(global_opts: CommandGlobalOptions, seed: Option<i64>, page_size: Option<u32>, tag: Option<String>, extended: bool) -> () {
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

    let logout_after_operation = matches!(vault_url, crate::tools::VaultURI::LoginURI(_));
    let login_result = ensure_login(vault_url, None, global_opts.verbose).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get tags

    let tags_res = api_call_get_tags(vault_url.clone()).await;

    if tags_res.is_err() {
        print_request_error(tags_res.err().unwrap());
        process::exit(1);
    }

    let tags = tags_map_from_list(&tags_res.unwrap());

    // Params

    let seed_param = seed.unwrap_or(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(
            Duration::from_secs(0)
        ).as_millis() as i64
    );

    let page_size_param = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let mut tag_param: Option<String> = None;

    if tag.is_some() {
        let tag_name = tag.unwrap();

        if is_identifier(&tag_name) {
            let tag_id_res = parse_identifier(&tag_name);

            match tag_id_res {
                Ok(tag_id) => {
                    if tags.contains_key(&tag_id) {
                        tag_param = Some((*tags.get(&tag_id).unwrap()).clone());
                    } else {
                        tag_param = Some(tag_name);
                    }
                },
                Err(_) => {
                    tag_param = Some(tag_name);
                },
            }
        } else {
            tag_param = Some(tag_name);
        }
    }

    // Call API

    let api_res = api_call_random(vault_url.clone(), tag_param, seed_param, page_size_param).await;

    match api_res {
        Ok(random_result) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;
        
                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            
            let page_size = random_result.page_size;
            let seed = random_result.seed;
            let page_items = random_result.page_items.len();

            println!("page size: {page_size}");
            println!("seed: {seed}");
            println!("items retrieved: {page_items}");

            if !extended {
                let table_head: Vec<String> = vec!["Id".to_string(), "Type".to_string(), "Title".to_string()];
                let mut table_body: Vec<Vec<String>> = iter::repeat_with(|| iter::repeat_with(|| "".to_string()).take(table_head.len()).collect()).take(page_items).collect();

                for (i, item) in random_result.page_items.iter().enumerate() {
                    table_body[i][0] = identifier_to_string(item.id).clone();
                    table_body[i][1] = item.media_type.to_string();
                    table_body[i][2] = item.title.clone();
                }

                print_table(&table_head, &table_body);
            } else {
                let table_head: Vec<String> = vec!["Id".to_string(), "Type".to_string(), "Title".to_string(), "Description".to_string(), "Tags".to_string(), "Duration".to_string()];
                let mut table_body: Vec<Vec<String>> = iter::repeat_with(|| iter::repeat_with(|| "".to_string()).take(table_head.len()).collect()).take(page_items).collect();

                for (i, item) in random_result.page_items.iter().enumerate() {
                    table_body[i][0] = identifier_to_string(item.id).clone();
                    table_body[i][1] = item.media_type.to_string();
                    table_body[i][2] = item.title.clone();
                    table_body[i][3] = item.description.clone();

                    table_body[i][4] = tags_names_from_ids(&item.tags, &tags).join(", ");

                    table_body[i][5] = render_media_duration(item.media_type, item.duration);
                }

                print_table(&table_head, &table_body);
            }
        },
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
        },
    }
}
