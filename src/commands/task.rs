// Task command

use std::process;

use clap::Subcommand;
use hyper::StatusCode;
use tokio::{
    io::{self, AsyncReadExt},
    task,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    api::{api_call_get_task, api_call_get_tasks},
    models::{get_task_remaining_time_string, get_task_status_string, get_task_type_string},
    tools::{
        ensure_login, identifier_to_string, parse_identifier, parse_vault_uri, print_table,
        to_csv_string, VaultURI,
    },
};

use super::{get_vault_url, logout::do_logout, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum TaskCommand {
    /// Lists current existing tasks
    #[clap(alias("ls"))]
    List {
        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Monitors tasks
    Monitor,

    /// Get task status
    Get {
        /// Task identifier
        task: String,
    },

    /// Waits for a task to finish, monitoring its status
    Wait {
        /// Task identifier
        task: String,
    },
}

pub async fn run_task_cmd(global_opts: CommandGlobalOptions, cmd: TaskCommand) {
    match cmd {
        TaskCommand::List { csv } => {
            run_cmd_list_tasks(global_opts, csv).await;
        }
        TaskCommand::Monitor => {
            run_cmd_monitor_tasks(global_opts).await;
        }
        TaskCommand::Get { task } => {
            run_cmd_get_task(global_opts, task).await;
        }
        TaskCommand::Wait { task } => {
            run_cmd_wait_for_task(global_opts, task).await;
        }
    }
}

pub async fn run_cmd_list_tasks(global_opts: CommandGlobalOptions, csv: bool) {
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

    let api_res = api_call_get_tasks(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(tasks) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let total = tasks.len();
            let running = tasks.iter().filter(|t| t.running).count();
            let pending = total - running;

            println!("total: {total}, running: {running}, pending: {pending}");

            if csv {
                println!();
                println!(
                    "\"Task ID\",\"Type\",\"Media\",\"Status\",\"Remaining time (Estimated)\""
                );

                for task in tasks {
                    let row_id = identifier_to_string(task.id);
                    let row_type = to_csv_string(&get_task_type_string(&task));
                    let row_media = identifier_to_string(task.media_id);
                    let row_status = to_csv_string(&get_task_status_string(&task));
                    let row_estimated_time = to_csv_string(&get_task_remaining_time_string(&task));
                    println!("{row_id},{row_type},{row_media},{row_status},{row_estimated_time}");
                }
            } else {
                let table_head: Vec<String> = vec![
                    "Task ID".to_string(),
                    "Type".to_string(),
                    "Media".to_string(),
                    "Status".to_string(),
                    "Remaining time (Estimated)".to_string(),
                ];

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(total);

                for task in tasks {
                    table_body.push(vec![
                        identifier_to_string(task.id),
                        get_task_type_string(&task),
                        identifier_to_string(task.media_id),
                        get_task_status_string(&task),
                        get_task_remaining_time_string(&task),
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

pub async fn run_cmd_get_task(global_opts: CommandGlobalOptions, task: String) {
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

    // Params

    let task_id_res = parse_identifier(&task);
    let task_id: u64 = match task_id_res {
        Ok(id) => id,
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
            eprintln!("Invalid task identifier specified.");
            process::exit(1);
        }
    };

    // Call API

    let api_res = api_call_get_task(&vault_url, task_id, global_opts.debug).await;

    match api_res {
        Ok(task) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let task_id = identifier_to_string(task.id);

            println!("Task {task_id}:");

            let task_type = &get_task_type_string(&task);

            println!("  Type: {task_type}");

            let media_id = identifier_to_string(task.media_id);

            println!("  Media: {media_id}");

            let status = get_task_status_string(&task);

            println!("  Status: {status}");

            let estimated_time = get_task_remaining_time_string(&task);

            println!("  Remaining time (Estimated): {estimated_time}");
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

pub async fn run_cmd_monitor_tasks(global_opts: CommandGlobalOptions) {
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

    // Spawn term thread

    spawn_termination_thread(
        global_opts.clone(),
        logout_after_operation,
        vault_url.clone(),
    );

    // Call API

    let mut monitoring_started = false;

    loop {
        let api_res = api_call_get_tasks(&vault_url, false).await;

        match api_res {
            Ok(tasks) => {
                monitoring_started = true;

                eprint!("\x1B[2J\x1B[1;1H"); // Clear screen and position the cursor at the top

                eprintln!("Monitoring tasks. Press Enter or Ctrl + C to exit.");

                let total = tasks.len();
                let running = tasks.iter().filter(|t| t.running).count();
                let pending = total - running;

                eprintln!("total: {total}, running: {running}, pending: {pending}");

                let table_head: Vec<String> = vec![
                    "Task ID".to_string(),
                    "Type".to_string(),
                    "Media".to_string(),
                    "Status".to_string(),
                    "Remaining time (Estimated)".to_string(),
                ];

                let (_, term_rows) = term_size::dimensions().unwrap_or((0, 0));

                let min_req_rows = 6; // Title + counts, start_table_separator + header + begin_body_separator + end_body_separator

                let mut allowed_rows: usize = 1;

                if term_rows > min_req_rows {
                    allowed_rows = term_rows - min_req_rows;
                }

                let mut table_body: Vec<Vec<String>> = Vec::with_capacity(allowed_rows);

                for (i, task) in tasks.into_iter().enumerate() {
                    if i >= allowed_rows {
                        break;
                    }

                    table_body.push(vec![
                        identifier_to_string(task.id),
                        get_task_type_string(&task),
                        identifier_to_string(task.media_id),
                        get_task_status_string(&task),
                        get_task_remaining_time_string(&task),
                    ]);
                }

                print_table(&table_head, &table_body, true);
            }
            Err(e) => match e {
                crate::tools::RequestError::Api {
                    status,
                    code: _,
                    message: _,
                } => {
                    if status == StatusCode::UNAUTHORIZED && monitoring_started {
                        if logout_after_operation {
                            do_logout(&global_opts, &vault_url)
                                .await
                                .unwrap_or(());
                        }
                        process::exit(0);
                    } else {
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
                crate::tools::RequestError::StatusCode(_)
                | crate::tools::RequestError::Json {
                    message: _,
                    body: _,
                }
                | crate::tools::RequestError::Hyper(_)
                | crate::tools::RequestError::FileSystem(_) => {
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
            },
        }

        // Wait
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

pub fn spawn_termination_thread(
    global_opts: CommandGlobalOptions,
    logout_after_operation: bool,
    vault_url: VaultURI,
) {
    task::spawn(async move {
        let mut stdin = io::stdin();

        loop {
            let r = stdin.read_u8().await;

            match r {
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

                    process::exit(0);
                }
                Err(_) => {
                    process::exit(1);
                }
            }
        }
    });
}

pub async fn run_cmd_wait_for_task(global_opts: CommandGlobalOptions, task: String) {
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

    // Params

    let task_id_res = parse_identifier(&task);
    let task_id: u64 = match task_id_res {
        Ok(id) => id,
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
            eprintln!("Invalid task identifier specified.");
            process::exit(1);
        }
    };

    let task_id_str = identifier_to_string(task_id);

    // Call API

    let mut task_found = false;
    let mut clear_line_str = "".to_string();

    loop {
        let api_res = api_call_get_task(&vault_url, task_id, false).await;

        match api_res {
            Ok(task) => {
                task_found = true;

                let task_id = identifier_to_string(task.id);
                let task_type = &get_task_type_string(&task);
                let media_id = identifier_to_string(task.media_id);
                let status = get_task_status_string(&task);
                let estimated_time = get_task_remaining_time_string(&task);

                eprint!("\r{clear_line_str}");

                let formatted_str = format!("Task {task_id} | Type: {task_type} | Media: {media_id} | Status: {status} | Remaining time (Estimated): {estimated_time}");

                clear_line_str = " ".repeat(formatted_str.width());

                eprint!("\r{formatted_str}");
            }
            Err(e) => {
                if logout_after_operation {
                    let logout_res = do_logout(&global_opts, &vault_url).await;

                    match logout_res {
                        Ok(_) => {}
                        Err(_) => {
                            process::exit(1);
                        }
                    }
                }
                match &e {
                    crate::tools::RequestError::StatusCode(_) => {
                        print_request_error(e);
                    }
                    crate::tools::RequestError::Api {
                        status,
                        code: _,
                        message: _,
                    } => {
                        if *status == StatusCode::NOT_FOUND {
                            eprint!("\r{clear_line_str}");
                            if task_found {
                                eprintln!("\rTask {task_id_str} completed!");
                            } else {
                                eprintln!("\rTask {task_id_str} not found, or already completed.");
                            }
                            process::exit(0);
                        } else {
                            print_request_error(e);
                        }
                    }
                    crate::tools::RequestError::Hyper(_)
                    | crate::tools::RequestError::FileSystem(_) => {
                        print_request_error(e);
                    }
                    crate::tools::RequestError::Json {
                        message: _,
                        body: _,
                    } => {
                        print_request_error(e);
                    }
                }
                process::exit(1);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}
