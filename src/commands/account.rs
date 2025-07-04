// Account command

use std::process;

use clap::Subcommand;

use crate::{
    api::{
        api_call_change_password, api_call_change_username, api_call_context,
        api_call_create_account, api_call_delete_account, api_call_disable_tfa,
        api_call_enable_totp, api_call_get_security_settings, api_call_get_totp_settings,
        api_call_list_accounts, api_call_set_security_settings, api_call_update_account,
    },
    models::{
        AccountCreateBody, AccountDeleteBody, AccountSetSecuritySettingsBody, AccountUpdateBody,
        ChangePasswordBody, ChangeUsernameBody, TfaDisableBody, TimeOtpAlgorithm,
        TimeOtpEnableBody, TimeOtpOptions, TimeOtpPeriod,
    },
    tools::{
        ask_user, ask_user_password, ensure_login, parse_vault_uri, print_table,
        request_auth_confirmation_password, request_auth_confirmation_tfa, to_csv_string,
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

    /// Gets account security settings
    GetSecuritySettings,

    /// Sets auth confirmation options
    SetAuthConfirmation {
        /// Set to 'true' to enable auth confirmation, Set it to 'false' to disable it
        auth_confirmation: String,

        /// Prefer using the account password instead of two factor authentication
        #[arg(long)]
        prefer_password: bool,

        /// Period (seconds) to remember the last auth confirmation
        #[arg(long)]
        period_seconds: Option<i32>,
    },

    /// Gets TOTP settings in order to enable two factor authentication
    GetTotpSettings {
        /// TOTP issuer (to be added th the URL)
        #[arg(long)]
        issuer: Option<String>,

        /// TOTP account (to be added th the URL)
        #[arg(long)]
        account: Option<String>,

        /// Hashing algorithm (sha-1, sha-256 or sha-512)
        #[arg(long)]
        algorithm: Option<String>,

        /// TOTP period (30s, 60s or 120s)
        #[arg(long)]
        period: Option<String>,

        /// Allows clock skew of 1 period
        #[arg(long)]
        allow_skew: bool,
    },

    /// Enables two factor authentication
    EnableTfa {
        /// Two factor authentication method (from the settings command result)
        method: String,

        /// Two factor authentication secret
        secret: String,
    },

    /// Disables two factor authentication
    DisableTfa,

    /// Lists accounts
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
        AccountCommand::GetSecuritySettings => {
            run_cmd_get_account_security(global_opts).await;
        }
        AccountCommand::SetAuthConfirmation {
            auth_confirmation,
            prefer_password,
            period_seconds,
        } => {
            run_cmd_set_account_security(
                global_opts,
                auth_confirmation,
                prefer_password,
                period_seconds,
            )
            .await;
        }
        AccountCommand::GetTotpSettings {
            issuer,
            account,
            algorithm,
            period,
            allow_skew,
        } => {
            run_cmd_get_totp_settings(global_opts, issuer, account, algorithm, period, allow_skew)
                .await;
        }
        AccountCommand::EnableTfa { method, secret } => {
            run_cmd_enable_tfa(global_opts, method, secret).await;
        }
        AccountCommand::DisableTfa => {
            run_cmd_disable_tfa(global_opts).await;
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

pub async fn run_cmd_get_account_security(global_opts: CommandGlobalOptions) {
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

    let api_res = api_call_get_security_settings(&vault_url, global_opts.debug).await;

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

            println!("---------------------------");

            if res.auth_confirmation {
                println!("Auth confirmation: Enabled");

                if res.auth_confirmation_method == "pw" {
                    println!("Auth confirmation preferred method: Account password");
                } else {
                    println!("Auth confirmation preferred method: Two factor authentication");
                }

                println!(
                    "Auth confirmation period: {} seconds",
                    res.auth_confirmation_period_seconds
                );
            } else {
                println!("Auth confirmation: Disabled");
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

pub async fn run_cmd_set_account_security(
    global_opts: CommandGlobalOptions,
    auth_confirmation: String,
    prefer_password: bool,
    period_seconds: Option<i32>,
) {
    let auth_confirmation_bool = match auth_confirmation.to_lowercase().as_str() {
        "0" | "false" | "no" => false,
        "1" | "true" | "yes" => true,
        _ => {
            eprintln!("Invalid argument: Set it to TRUE or FALSE");
            process::exit(1);
        }
    };

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

    let api_res = api_call_set_security_settings(
        &vault_url,
        AccountSetSecuritySettingsBody {
            auth_confirmation: auth_confirmation_bool,
            auth_confirmation_method: if prefer_password {
                "pw".to_string()
            } else {
                "tfa".to_string()
            },
            auth_confirmation_period_seconds: period_seconds.unwrap_or(120),
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

            eprintln!("Successfully updated account security settings");
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

pub async fn run_cmd_get_totp_settings(
    global_opts: CommandGlobalOptions,
    issuer: Option<String>,
    account: Option<String>,
    algorithm: Option<String>,
    period: Option<String>,
    allow_skew: bool,
) {
    let hash_algorithm = match algorithm {
        Some(a) => match TimeOtpAlgorithm::parse(&a) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Invalid hash algorithm. Valid ones are: sha-1, sha-256, sha-512");
                process::exit(1);
            }
        },
        None => TimeOtpAlgorithm::Sha1,
    };

    let time_period = match period {
        Some(a) => match TimeOtpPeriod::parse(&a) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Invalid period. Valid ones are: 30s, 60s, 120s");
                process::exit(1);
            }
        },
        None => TimeOtpPeriod::P30,
    };

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

    let api_res = api_call_get_totp_settings(
        &vault_url,
        TimeOtpOptions {
            issuer,
            account,
            algorithm: hash_algorithm,
            period: time_period,
            skew: allow_skew,
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

            println!("---------------------------");

            println!("Method: {}", res.method);
            println!("Secret: {}", res.secret);
            println!("URL: {}", res.url);

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

pub async fn run_cmd_enable_tfa(global_opts: CommandGlobalOptions, method: String, secret: String) {
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

    // Ask for confirmation

    let confirmation_pw = request_auth_confirmation_password().await;
    let confirmation_tfa = request_auth_confirmation_tfa().await;

    // Call API

    let api_res = api_call_enable_totp(
        &vault_url,
        TimeOtpEnableBody {
            method,
            secret,
            password: confirmation_pw,
            code: confirmation_tfa,
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

            eprintln!("Successfully enabled two factor authentication");
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

pub async fn run_cmd_disable_tfa(global_opts: CommandGlobalOptions) {
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

    // Ask for confirmation

    let confirmation_tfa = request_auth_confirmation_tfa().await;

    // Call API

    let api_res = api_call_disable_tfa(
        &vault_url,
        TfaDisableBody {
            code: confirmation_tfa,
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

            eprintln!("Successfully disabled two factor authentication");
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
