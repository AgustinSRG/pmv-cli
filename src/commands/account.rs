// Account command

use std::{process, iter};

use clap::Subcommand;

use crate::{tools::{ensure_login, parse_vault_uri, ask_user_password, print_table}, api::{api_call_context, api_call_change_username, api_call_change_password, api_call_list_accounts}, models::{Credentials, ChangePasswordBody}};

use super::{CommandGlobalOptions, logout::do_logout, get_vault_url, print_request_error};

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
    List,

    /// Creates new account
    Create {
        /// Username for the new account
        username: String,

        /// Allows the new account to modify the vault
        #[arg(short, long)]
        allow_write: bool,
    },

    Delete {
        /// Username of the account to delete
        username: String,
    },
}

pub async fn run_account_cmd(global_opts: CommandGlobalOptions, cmd: AccountCommand) -> () {
    match cmd {
        AccountCommand::Context => {
            run_cmd_context(global_opts).await;
        },
        AccountCommand::ChangeUsername { username } => {
            run_cmd_change_username(global_opts, username).await;
        },
        AccountCommand::ChangePassword => {
            run_cmd_change_password(global_opts).await;
        },
        AccountCommand::List => {
            run_cmd_list_accounts(global_opts).await;
        },
        AccountCommand::Create { username, allow_write } => todo!(),
        AccountCommand::Delete { username } => todo!(),
    }
}

pub async fn run_cmd_context(global_opts: CommandGlobalOptions) -> () {
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

    // Call API

    let api_res = api_call_context(vault_url.clone()).await;

    match api_res {
        Ok(context) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;
        
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

pub async fn run_cmd_change_username(global_opts: CommandGlobalOptions, username: String) -> () {
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
    let base_url = vault_url.to_base_url();

    // Ask password

    eprintln!("Input password for vault: {base_url}");
    eprintln!("Password confirmation is required for this action");
    let password = ask_user_password("Password: ".to_string()).await.unwrap_or("".to_string());

    // Call API

    let api_res = api_call_change_username(vault_url.clone(), Credentials {
        username: username.clone(),
        password,
    }).await;

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

            if global_opts.verbose {
                eprintln!("Successfully changed account username to: {username}");
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

pub async fn run_cmd_change_password(global_opts: CommandGlobalOptions) -> () {
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
    let base_url = vault_url.to_base_url();

    // Ask new password

    let new_password = ask_user_password("New password: ".to_string()).await.unwrap_or("".to_string());
    let new_password_c = ask_user_password("Confirm new password: ".to_string()).await.unwrap_or("".to_string());

    if new_password != new_password_c {
        if logout_after_operation {
            let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;
    
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
            let logout_res = do_logout(global_opts.clone(), vault_url.clone()).await;
    
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
    let password = ask_user_password("Password: ".to_string()).await.unwrap_or("".to_string());

    // Call API

    let api_res = api_call_change_password(vault_url.clone(), ChangePasswordBody {
        old_password: password,
        password: new_password,
    }).await;

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

            if global_opts.verbose {
                eprintln!("Successfully changed account password");
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

pub async fn run_cmd_list_accounts(global_opts: CommandGlobalOptions) -> () {
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

    // Call API

    let api_res = api_call_list_accounts(vault_url.clone()).await;

    match api_res {
        Ok(accounts) => {
            if logout_after_operation {
                let logout_res = do_logout(global_opts, vault_url.clone()).await;
        
                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            
            let total = accounts.len();

            println!("total: {total}");

            let table_head: Vec<String> = vec!["Username".to_string(), "Permissions".to_string()];

            let mut table_body: Vec<Vec<String>> = iter::repeat_with(|| iter::repeat_with(|| "".to_string()).take(2).collect()).take(accounts.len()).collect();

            for (i, account) in accounts.iter().enumerate() {
                table_body[i][0] = account.username.clone();
                let write = account.write;

                if write {
                    table_body[i][1] = "read, write".to_string();
                } else {
                    table_body[i][1] = "read".to_string();
                } 
            }

            print_table(&table_head, &table_body);
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
