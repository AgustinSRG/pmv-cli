// Random media list command

use std::{process, time::Duration};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::tools::{render_media_duration, to_csv_string};
use crate::{
    api::{api_call_get_tags, api_call_random},
    models::{tags_map_from_list, tags_names_from_ids},
    tools::{
        ensure_login, identifier_to_string, is_identifier, parse_identifier, parse_vault_uri,
        print_table,
    },
};

use super::{get_vault_url, logout::do_logout, print_request_error, CommandGlobalOptions};

const DEFAULT_PAGE_SIZE: u32 = 10;

pub async fn run_cmd_random(
    global_opts: CommandGlobalOptions,
    seed: Option<i64>,
    page_size: Option<u32>,
    tag: Option<String>,
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

    let tags = tags_map_from_list(&tags_res.unwrap());

    // Params

    let seed_param = seed.unwrap_or(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as i64,
    );

    let page_size_param = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let mut tag_param: Option<String> = None;

    if let Some(tag_name) = tag {
        if is_identifier(&tag_name) {
            let tag_id_res = parse_identifier(&tag_name);

            match tag_id_res {
                Ok(tag_id) => {
                    if tags.contains_key(&tag_id) {
                        tag_param = Some((*tags.get(&tag_id).unwrap()).clone());
                    } else {
                        tag_param = Some(tag_name);
                    }
                }
                Err(_) => {
                    tag_param = Some(tag_name);
                }
            }
        } else {
            tag_param = Some(tag_name);
        }
    }

    // Call API

    let api_res = api_call_random(
        &vault_url,
        tag_param,
        seed_param,
        page_size_param,
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(random_result) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

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

            if csv {
                println!();
                if !extended {
                    println!("\"Id\",\"Type\",\"Title\"");

                    for item in random_result.page_items {
                        let row_id = item.id.to_string();
                        let row_type = to_csv_string(&item.media_type.to_type_string());
                        let row_title = to_csv_string(&item.title);
                        println!("{row_id},{row_type},{row_title}");
                    }
                } else {
                    println!("\"Id\",\"Type\",\"Title\",\"Description\",\"Tags\",\"Duration\"");

                    for item in random_result.page_items {
                        let row_id = item.id.to_string();
                        let row_type = to_csv_string(&item.media_type.to_type_string());
                        let row_title = to_csv_string(&item.title);
                        let row_description = to_csv_string(&item.description);
                        let row_tags =
                            to_csv_string(&tags_names_from_ids(&item.tags, &tags).join(" "));
                        let row_duration =
                            render_media_duration(item.media_type, item.duration.unwrap_or(0.0));

                        println!("{row_id},{row_type},{row_title},{row_description},{row_tags},{row_duration}");
                    }
                }
            } else if !extended {
                let table_head: Vec<String> =
                    vec!["Id".to_string(), "Type".to_string(), "Title".to_string()];
                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(page_items);

                for item in random_result.page_items {
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
                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(page_items);

                for item in random_result.page_items {
                    table_body.push(vec![
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
