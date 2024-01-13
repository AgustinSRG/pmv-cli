// Invites command

use std::process;

use clap::Subcommand;

use crate::{tools::{parse_vault_uri, ensure_login, duration_to_string, to_csv_string, format_date, print_table}, api::{api_call_check_invite, api_call_generate_invite, api_call_clear_invite, api_call_delete_invited_session, api_call_list_invited_sessions}, commands::logout::do_logout, models::InviteCodeGenerateBody};

use super::{CommandGlobalOptions, get_vault_url, print_request_error};

#[derive(Subcommand)]
pub enum InvitesCommand {
    /// Prints the current invite code, if any
    Check,

    /// Generates a new invite code
    Generate {
        /// Session duration. Can be: day, week, month or year
        #[arg(short = 'D', long)]
        duration: Option<String>,
    },

    /// Clears the current invite code
    Clear,

    /// List active invited sessions
    #[clap(alias("ls"))]
    ListSessions {
        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Closes an invited session
    CloseSession {
        /// Session index
        index: u64,
    },
}

pub async fn run_invites_cmd(global_opts: CommandGlobalOptions, cmd: InvitesCommand) {
    match cmd {
        InvitesCommand::Check => {
            run_cmd_invites_check(global_opts).await;
        },
        InvitesCommand::Generate { duration } => {
            run_cmd_invites_generate(global_opts, duration).await;
        },
        InvitesCommand::Clear => {
            run_cmd_invites_clear(global_opts).await;
        },
        InvitesCommand::ListSessions { csv } => {
            run_cmd_invites_list_sessions(global_opts, csv).await;
        }
        InvitesCommand::CloseSession { index } => {
            run_cmd_invites_close_session(global_opts, index).await;
        },
    }
}

pub async fn run_cmd_invites_check(global_opts: CommandGlobalOptions) {
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

    let api_res = api_call_check_invite(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(invite_code_status) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            match invite_code_status.has_code {
                true => {
                    println!("---------------------------");

                    if let Some(code) = invite_code_status.code {
                        println!("Invite code: {code}");
                    }

                    if let Some(duration) = invite_code_status.duration {
                        let duration_days = duration / (24 * 60 * 60 * 1000);

                        if duration_days == 1 {
                            println!("Duration: 1 day");
                        } else {
                            println!("Duration: {duration_days} days");
                        }
                    }

                    if let Some(expiration_remaining) = invite_code_status.expiration_remaining {
                        if expiration_remaining <= 0 {
                            println!("THIS CODE HAS ALREADY EXPIRED");
                        } else {
                            let d_seconds = (expiration_remaining as f64) / 1000.0;

                            let d_str = duration_to_string(d_seconds);

                            println!("Expires in: {d_str}");
                        }
                    }

                    println!("---------------------------");
                },
                false => {
                    println!("Your account does not have any active invite code");
                },
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

pub async fn run_cmd_invites_generate(
    global_opts: CommandGlobalOptions,
    duration: Option<String>,
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

    let api_res = api_call_generate_invite(
        &vault_url,
        InviteCodeGenerateBody {
            duration: duration.unwrap_or("day".to_string()),
        },
        global_opts.debug,
    )
    .await;

    match api_res {
        Ok(invite_code_status) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            if let Some(code) = invite_code_status.code {
                println!("Invite code: {code}");
            }

            if let Some(duration) = invite_code_status.duration {
                let duration_days = duration / (24 * 60 * 60 * 1000);

                if duration_days == 1 {
                    println!("Duration: 1 day");
                } else {
                    println!("Duration: {duration_days} days");
                }
            }

            if let Some(expiration_remaining) = invite_code_status.expiration_remaining {
                if expiration_remaining <= 0 {
                    println!("THIS CODE HAS ALREADY EXPIRED");
                } else {
                    let d_seconds = (expiration_remaining as f64) / 1000.0;

                    let d_str = duration_to_string(d_seconds);

                    println!("Expires in: {d_str}");
                }
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

pub async fn run_cmd_invites_clear(
    global_opts: CommandGlobalOptions,
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

    let api_res = api_call_clear_invite(
        &vault_url,
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

            println!("Invite code successfully cleared");
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

pub async fn run_cmd_invites_list_sessions(global_opts: CommandGlobalOptions, csv: bool) {
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

    let api_res = api_call_list_invited_sessions(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(sessions) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let total = sessions.len();

            println!("total: {total}");

            if csv {
                println!();
                println!("\"Index\",\"Timestamp\",\"Expiration\"");

                for s in sessions {
                    let row_index = s.index;
                    let row_timestamp = to_csv_string(&format_date(s.timestamp));
                    let row_expiration = to_csv_string(&format_date(s.expiration));
                   
                    println!("{row_index},{row_timestamp},{row_expiration}");
                }
            } else {
                let table_head: Vec<String> =
                    vec!["Index".to_string(), "Timestamp".to_string(), "Expiration".to_string()];

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(total);

                for s in sessions {
                    let row_index = s.index;
                    table_body.push(vec![
                        format!("#{row_index}"),
                        to_csv_string(&format_date(s.timestamp)),
                        to_csv_string(&format_date(s.expiration)),
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

pub async fn run_cmd_invites_close_session(
    global_opts: CommandGlobalOptions,
    index: u64,
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

    let api_res = api_call_delete_invited_session(
        &vault_url,
        index,
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

            println!("Invited session successfully closed");
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
