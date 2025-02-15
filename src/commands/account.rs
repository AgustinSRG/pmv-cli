// Account command

use std::process;

use clap::Subcommand;

use crate::{
    api::{
        api_call_change_password, api_call_change_username, api_call_context,
        api_call_create_account, api_call_delete_account, api_call_list_accounts,
        api_call_update_account,
    },
    models::{
        AccountCreateBody, AccountDeleteBody, AccountUpdateBody, ChangePasswordBody,
        ChangeUsernameBody,
    },
    tools::{
        ask_user, ask_user_password, ensure_login, parse_vault_uri, print_table, to_csv_string,
    },
};

use super::{get_vault_url, logout::do_logout, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum AccountCommand {
    /// Prints account context to the standard output
    Context,

    /// Changes username (only for root account)
    ChangeUsername {
        /// Username to change into
        username: String,
    },

    /// Changes account password
    ChangePassword,

    /// List accounts
    #[clap(alias("ls"))]
    List {
        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Creates new account
    Create {
        /// Username for the new account
        username: String,

        /// Allows the new account to modify the vault
        #[arg(short, long)]
        allow_write: bool,
    },

    /// Updates an account
    Update {
        /// Username of the account
        username: String,

        /// New username for the account
        #[arg(short, long)]
        new_username: Option<String>,

        /// Allows the account to modify the vault
        #[arg(short, long)]
        allow_write: Option<bool>,
    },

    /// Deletes an existing account
    Delete {
        /// Username of the account to delete
        username: String,
    },
}

pub async fn run_account_cmd(global_opts: CommandGlobalOptions, cmd: AccountCommand) {
    match cmd {
        AccountCommand::Context => {
            run_cmd_context(global_opts).await;
        }
        AccountCommand::ChangeUsername { username } => {
            run_cmd_change_username(global_opts, username).await;
        }
        AccountCommand::ChangePassword => {
            run_cmd_change_password(global_opts).await;
        }
        AccountCommand::List { csv } => {
            run_cmd_list_accounts(global_opts, csv).await;
        }
        AccountCommand::Create {
            username,
            allow_write,
        } => {
            run_cmd_create_account(global_opts, username, allow_write).await;
        }
        AccountCommand::Update {
            username,
            new_username,
            allow_write,
        } => {
            run_cmd_update_account(global_opts, username, new_username, allow_write).await;
        }
        AccountCommand::Delete { username } => {
            run_cmd_delete_account(global_opts, username).await;
        }
    }
}

pub async fn run_cmd_context(global_opts: CommandGlobalOptions) {
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

    let api_res = api_call_context(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(context) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let res_username = context.username;
            let res_title = context.title.unwrap_or("".to_string());
            let res_root = context.root;
            let res_write = context.write;

            println!("---------------------------");

            println!("Username: {res_username}");
            if res_root {
                println!("Permissions: Vault Administrator");
            } else if res_write {
                println!("Permissions: Vault Read & Write");
            } else {
                println!("Permissions: Read Only");
            }

            if !res_title.is_empty() {
                println!("Vault title: {res_title}");
            }

            println!("---------------------------");
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

pub async fn run_cmd_change_username(global_opts: CommandGlobalOptions, username: String) {
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
    let base_url = vault_url.to_base_url();

    // Ask password

    eprintln!("Input password for vault: {base_url}");
    eprintln!("Password confirmation is required for this action");
    let password = ask_user_password("Password: ")
        .await
        .unwrap_or("".to_string());

    // Call API

    let api_res = api_call_change_username(
        &vault_url,
        ChangeUsernameBody {
            username: username.clone(),
            password,
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

            eprintln!("Successfully changed account username to: {username}");
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

pub async fn run_cmd_change_password(global_opts: CommandGlobalOptions) {
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
    let base_url = vault_url.to_base_url();

    // Ask new password

    let new_password = ask_user_password("New password: ")
        .await
        .unwrap_or("".to_string());
    let new_password_c = ask_user_password("Confirm new password: ")
        .await
        .unwrap_or("".to_string());

    if new_password != new_password_c {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: The passwords do not match");
        process::exit(1);
    }

    if new_password.is_empty() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: The password cannot be blank");
        process::exit(1);
    }

    // Ask password

    eprintln!("Input password for vault: {base_url}");
    eprintln!("Password confirmation is required for this action");
    let password = ask_user_password("Password: ")
        .await
        .unwrap_or("".to_string());

    // Call API

    let api_res = api_call_change_password(
        &vault_url,
        ChangePasswordBody {
            old_password: password,
            password: new_password,
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

            eprintln!("Successfully changed account password.");
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

pub async fn run_cmd_list_accounts(global_opts: CommandGlobalOptions, csv: bool) {
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

    let api_res = api_call_list_accounts(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(accounts) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let total = accounts.len();

            println!("total: {total}");

            if csv {
                println!();
                println!("\"Username\",\"Permissions\"");

                for account in accounts {
                    let row_username = to_csv_string(&account.username);
                    let row_permissions: String = if account.write {
                        to_csv_string("read, write")
                    } else {
                        to_csv_string("read")
                    };
                    println!("{row_username},{row_permissions}");
                }
            } else {
                let table_head: Vec<String> =
                    vec!["Username".to_string(), "Permissions".to_string()];

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(total);

                for account in accounts {
                    table_body.push(vec![
                        to_csv_string(&account.username),
                        if account.write {
                            to_csv_string("read, write")
                        } else {
                            to_csv_string("read")
                        },
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

pub async fn run_cmd_create_account(
    global_opts: CommandGlobalOptions,
    username: String,
    allow_write: bool,
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

    // Ask password for the new account

    eprintln!("Input a password for the new account: {username}");
    let new_password = ask_user_password("Password: ")
        .await
        .unwrap_or("".to_string());
    let new_password_c = ask_user_password("Confirm password: ")
        .await
        .unwrap_or("".to_string());

    if new_password != new_password_c {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: The passwords do not match");
        process::exit(1);
    }

    if new_password.is_empty() {
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        eprintln!("Error: The password cannot be blank");
        process::exit(1);
    }

    // Call API

    let api_res = api_call_create_account(
        &vault_url,
        AccountCreateBody {
            username: username.clone(),
            password: new_password,
            write: allow_write,
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

            eprintln!("Successfully created account: {username}");
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

pub async fn run_cmd_update_account(
    global_opts: CommandGlobalOptions,
    username: String,
    new_username: Option<String>,
    allow_write: Option<bool>,
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

    let api_res = api_call_update_account(
        &vault_url,
        AccountUpdateBody {
            username: username.clone(),
            new_username,
            write: allow_write,
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

            eprintln!("Successfully created account: {username}");
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

pub async fn run_cmd_delete_account(global_opts: CommandGlobalOptions, username: String) {
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

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to delete the vault account {username}?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
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

    // Call API

    let api_res = api_call_delete_account(
        &vault_url,
        AccountDeleteBody {
            username: username.clone(),
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

            eprintln!("Successfully deleted account: {username}");
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
