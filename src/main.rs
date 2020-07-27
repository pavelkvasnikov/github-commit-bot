#![warn(rust_2018_idioms)]

use std::env;
use std::collections::HashMap;
mod logger;
use logger::logger::initialize_logger;
use log::{info, warn};
use git2::*;
use std::fs::OpenOptions;
use std::io::Write;
mod constants;
use constants::*;
mod config;
use config::app_config::check_config;
use lazy_static::lazy_static;
use git2::build::RepoBuilder;
use std::path::Path;
use std::time::Duration;

lazy_static! {
    static ref CONFIG: HashMap<String, String> = {
        let mut m = HashMap::new();
          for argument in env::args() {
            let params: Vec<_> = argument.split('=').collect();
            if params.len() != 2 {
                warn!("Error string  - {:?}", params);
            } else {
                m.insert(params[0].to_string(), params[1].to_string());
            }
        }
    };
}

fn main() {
    let _logger_handle = initialize_logger();
    check_config(CONFIG.clone());
    let interval = Duration::from_millis(CONFIG[TIMEOUT].parse::<u64>().unwrap());

    let path_string = &CONFIG[PATH];
    let path = Path::new(&path_string);
    info!("Trying to open repo locally");
    let try_repo = Repository::open(path);

    if try_repo.is_err() {
        info!("Repo not found, trying to clone");
        let mut builder = RepoBuilder::new();
        let mut rco = RemoteCallbacks::new();
        rco.credentials(git_credentials_callback);
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(rco);
        builder.fetch_options(fetch_options);
        let try_clone = builder.clone(&CONFIG["repo"].clone(), path);
        if try_clone.is_err() {
            let error = try_clone.err().unwrap();
            warn!("error during cloning repo - {}", error.message());
            panic!("\n\nError class {:?}\nError message {:?}\nError code {:?}\n\n",
                   error.class(),
                   error.message(),
                   error.code());
        } else {
            info!("Cloned");
        }
    } else {
        info!("Repo opened");
    }

    let repo = Repository::open(path).unwrap();
    let file_path = format!("{}/{}", CONFIG["path"], CONFIG["file"]);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .append(true)
        .open(file_path);
    let mut unwrapped = file.unwrap();

    let mut remote = repo.find_remote("origin").unwrap();

    let mut po = PushOptions::new();
    let mut rco = RemoteCallbacks::new();
    rco.credentials(git_credentials_callback);
    po.remote_callbacks(rco);

    loop {
        let write_result = unwrapped.write_all(b"\nnewline");
        info!("Wrote to file - {:?}", write_result.unwrap());
        let write_result = unwrapped.sync_all();
        if write_result.is_err() {
            warn!("Error during writing to file - {:?}", write_result.err());
            panic!("Error during writing to file");
        }

        let mut index = repo.index().unwrap();
        let index_add_all_result = index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None);
        if index_add_all_result.is_err() {
            warn!("Error during add to index - {}", index_add_all_result.err().unwrap().message());
        }
        let index_write_result = index.write();
        if index_write_result.is_err() {
            warn!("Error during writing index - {}", index_write_result.err().unwrap().message());
        }
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let parent_commit = find_last_commit(&repo).unwrap();
        info!("Committing...");
        let commit_result = repo.commit(Option::Some("HEAD"),
                                        &git2::Signature::now(&CONFIG["username"].clone(), &CONFIG["email"].clone()).unwrap(),
                                        &git2::Signature::now(&CONFIG["username"].clone(), &CONFIG["email"].clone()).unwrap(),
                                        "test message",
                                        &tree,
                                        &[&parent_commit]);
        if commit_result.is_ok() {
            info!("Committed");
        } else {
            warn!("Not committed - {}", commit_result.err().unwrap().message());
        }

        info!("Pushing..");
        let push_result = remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut po));
        if push_result.is_ok() {
            info!("Pushed");
        } else {
            warn!("Not pushed {}", push_result.err().unwrap().message());
        }
        std::thread::sleep(interval);
    }
}

fn find_last_commit(repo: &Repository) -> Result<Commit<'_>, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn git_credentials_callback(
    _user: &str,
    user_from_url: Option<&str>,
    cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    let user = user_from_url.unwrap_or("git");
    if cred.contains(git2::CredentialType::USERNAME) {
        return git2::Cred::username(user);
    }
    let key_file = &CONFIG["ssh_private_key"];
    let key_file_pub = &CONFIG["ssh_pub_key"];
    git2::Cred::ssh_key(
        user,
        Some(std::path::Path::new(&key_file_pub)),
        std::path::Path::new(&key_file),
        None,
    )
}
