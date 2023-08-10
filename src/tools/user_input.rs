// User input functions

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::thread;
use tokio::io::AsyncBufReadExt;
use tokio::io::{self, BufReader};

pub async fn ask_user(prompt: &str) -> Result<String, ()> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let mut lines = reader.lines();

    eprint!("{prompt}");

    let line_res = lines.next_line().await;

    match line_res {
        Ok(line) => match line {
            Some(line_str) => Ok(line_str),
            None => Ok("".to_string()),
        },
        Err(_) => Err(()),
    }
}

struct PasswordPromptStatus {
    done: bool,
    password: String,
}
struct PasswordPrompt {
    prompt: String,
    status: Arc<Mutex<PasswordPromptStatus>>,
}

impl Future for PasswordPrompt {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let status = self.status.lock().unwrap();
        if status.done {
            Poll::Ready(status.password.clone())
        } else {
            let waker = cx.waker().clone();
            let prompt = self.prompt.clone();
            let self_arc = Arc::clone(&self.status);

            thread::spawn(move || {
                let password = rpassword::prompt_password(prompt).unwrap_or("".to_string());

                let mut status_changer = self_arc.lock().unwrap();

                status_changer.done = true;
                status_changer.password = password;

                waker.wake();
            });

            Poll::Pending
        }
    }
}

pub async fn ask_user_password(prompt: &str) -> Result<String, ()> {
    let pp = PasswordPrompt {
        prompt: prompt.to_string(),
        status: Arc::new(Mutex::new(PasswordPromptStatus {
            done: false,
            password: "".to_string(),
        })),
    };

    let password = pp.await;

    Ok(password)
}
