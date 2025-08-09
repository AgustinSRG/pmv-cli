// Home command

use std::process;

use clap::Subcommand;

use crate::{
    api::{
        api_call_delete_home_group, api_call_get_home_group_elements, api_call_get_home_groups,
        api_call_home_add_group, api_call_move_home_group, api_call_rename_home_group,
        api_call_set_home_group_elements,
    },
    commands::logout::do_logout,
    models::{
        HomePageAddGroupBody, HomePageElementRef, HomePageGroupMoveBody, HomePageGroupRenameBody,
        HomePageGroupSetElementsBody, HomePageGroupType,
    },
    tools::{ensure_login, parse_vault_uri, print_table},
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum HomeCommand {
    /// Gets the groups in the home page
    GetGroups,

    /// Adds a home page group
    AddGroup {
        /// A name for the group
        name: String,

        /// The type of group (CUSTOM, RECENT_MEDIA or RECENT_ALBUMS)
        group_type: String,

        /// Add this option in order to add the group at the top of the list
        #[arg(long)]
        prepend: bool,
    },

    /// Gets the elements of a group
    GetGroupElements {
        /// ID of the group
        id: u64,

        /// Add this option to print the elements as references
        #[arg(long)]
        as_refs: bool,
    },

    /// Sets the elements of a group
    SetGroupElements {
        /// ID of the group
        id: u64,

        /// List of group elements, as references, separated by commas. For media elements use M{ID} and for albums use A{ID}. Example: A12, M6, M8
        elements: String,
    },

    /// Renames an existing group
    RenameGroup {
        /// ID of the group
        id: u64,

        /// A name for the group
        name: String,
    },

    /// Moves an existing group to another position
    MoveGroup {
        /// ID of the group
        id: u64,

        /// The position to move the group
        position: u32,
    },

    /// Deletes an existing group
    DeleteGroup {
        /// ID of the group
        id: u64,
    },
}

pub async fn run_home_cmd(global_opts: CommandGlobalOptions, cmd: HomeCommand) {
    match cmd {
        HomeCommand::GetGroups => {
            run_cmd_home_get_groups(global_opts).await;
        }
        HomeCommand::AddGroup {
            name,
            group_type,
            prepend,
        } => {
            run_cmd_home_add_group(global_opts, name, group_type, prepend).await;
        }
        HomeCommand::GetGroupElements { id, as_refs } => {
            run_cmd_home_get_group_elements(global_opts, id, as_refs).await;
        }
        HomeCommand::SetGroupElements { id, elements } => {
            run_cmd_home_set_group_elements(global_opts, id, elements).await;
        }
        HomeCommand::RenameGroup { id, name } => {
            run_cmd_home_rename_group(global_opts, id, name).await;
        }
        HomeCommand::MoveGroup { id, position } => {
            run_cmd_home_move_group(global_opts, id, position).await;
        }
        HomeCommand::DeleteGroup { id } => {
            run_cmd_home_delete_group(global_opts, id).await;
        }
    }
}

pub async fn run_cmd_home_get_groups(global_opts: CommandGlobalOptions) {
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

    // Call API

    let api_res = api_call_get_home_groups(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(groups) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let table_head: Vec<String> = vec![
                "Id".to_string(),
                "Type".to_string(),
                "Name".to_string(),
                "Custom elements".to_string(),
            ];
            let mut table_body: Vec<Vec<String>> = Vec::with_capacity(groups.len());

            for group in groups {
                table_body.push(vec![
                    group.id.to_string(),
                    group.group_type.as_string(),
                    group.name.clone().unwrap_or("".to_string()),
                    group.elements_count.unwrap_or(0).to_string(),
                ]);
            }

            print_table(&table_head, &table_body, false);
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

pub async fn run_cmd_home_add_group(
    global_opts: CommandGlobalOptions,
    name: String,
    group_type: String,
    prepend: bool,
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

    let group_type_model = match HomePageGroupType::from_string(&group_type) {
        Some(t) => t,
        None => {
            eprintln!(
                "Invalid group type: {}. Valid ones are: CUSTOM, RECENT_MEDIA and RECENT_ALBUMS",
                group_type
            );
            process::exit(1);
        }
    };

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Call API

    let api_res = api_call_home_add_group(
        &vault_url,
        HomePageAddGroupBody {
            name: Some(name.clone()),
            group_type: group_type_model,
            prepend,
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(res) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("Successfully added group {} to home page", res.id);
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

pub async fn run_cmd_home_get_group_elements(
    global_opts: CommandGlobalOptions,
    id: u64,
    as_refs: bool,
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

    // Call API

    let api_res = api_call_get_home_group_elements(&vault_url, id, global_opts.debug).await;

    match api_res {
        Ok(elements) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            if as_refs {
                let element_refs = HomePageElementRef::from_home_page_elements(&elements);
                let list_string = HomePageElementRef::as_list_string(&element_refs);

                println!("{}", list_string);
            } else {
                let table_head: Vec<String> =
                    vec!["Id".to_string(), "Type".to_string(), "Title".to_string()];
                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(elements.len());

                for element in elements {
                    if !element.is_valid_element() {
                        continue;
                    }

                    table_body.push(vec![
                        element.get_element_id().to_string(),
                        element.get_element_type().as_string(),
                        element.get_element_title(),
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

pub async fn run_cmd_home_set_group_elements(
    global_opts: CommandGlobalOptions,
    id: u64,
    elements: String,
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

    let elements_refs = match HomePageElementRef::from_list_string(&elements) {
        Ok(r) => r,
        Err(pos) => {
            eprintln!("Invalid element reference as position {}", pos);
            process::exit(1);
        }
    };

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Call API

    let api_res = api_call_set_home_group_elements(
        &vault_url,
        id,
        HomePageGroupSetElementsBody {
            elements: elements_refs,
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("Successfully set elements for group {}", id);
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

pub async fn run_cmd_home_rename_group(global_opts: CommandGlobalOptions, id: u64, name: String) {
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

    // Call API

    let api_res = api_call_rename_home_group(
        &vault_url,
        id,
        HomePageGroupRenameBody {
            name: Some(name.clone()),
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("Successfully renamed group {} to '{}'", id, name);
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

pub async fn run_cmd_home_move_group(global_opts: CommandGlobalOptions, id: u64, position: u32) {
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

    // Call API

    let api_res = api_call_move_home_group(
        &vault_url,
        id,
        HomePageGroupMoveBody { position },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("Successfully moved group {} to position {}", id, position);
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

pub async fn run_cmd_home_delete_group(global_opts: CommandGlobalOptions, id: u64) {
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

    // Call API

    let api_res = api_call_delete_home_group(&vault_url, id, global_opts.debug).await;

    match api_res {
        Ok(_) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("Successfully deleted group {} from the home page", id);
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
