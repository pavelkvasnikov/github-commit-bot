#![warn(rust_2018_idioms)]

use std::env;
use std::collections::HashMap;
use tokio::time::{Duration};
mod logger;
use logger::logger::initialize_logger;
use log::{info, warn};
use git2::*;

use std::fs::OpenOptions;
use std::io::Write;

const TIMEOUT:u64 = 3500;

use lazy_static::lazy_static;
use git2::build::RepoBuilder;
use std::path::Path;
lazy_static! {
    static ref CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
          for argument in env::args() {
            let params: Vec<_> = argument.split('=').collect();
            if params.len() != 2 {
                warn!("Error string  - {:?}", params);
            } else {
                m.insert(params[0].to_string(), params[1].to_string());
            }
        }
        m
    };
}




#[tokio::main]
async fn main() {
    let _logger_handle = initialize_logger();

    let mut interval = tokio::time::interval(Duration::from_millis(TIMEOUT));
    let mut is_opened = false;


    loop {
        interval.tick().await;
        if !is_opened {
            info!("Trying to open repo...");
        }
        let path_string = CREDENTIALS["path"].clone();
        let path = Path::new(&path_string);
        let try_repo = Repository::open(path);

        if !is_opened && try_repo.is_err() {
            info!("Repo not find, trying to clone");
            let mut builder = RepoBuilder::new();
            let mut rco = RemoteCallbacks::new();
            rco.credentials(git_credentials_callback);
            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(rco);
            builder.fetch_options(fetch_options);


            let try_clone = builder.clone(&CREDENTIALS["repo"].clone(), path);
            if try_clone.is_err() {
                warn!("error during cloning repo - {}", try_clone.err().unwrap().message());
            } else {
                info!("Cloned");
            }
        } else {
            info!("Repo opened");
            is_opened = true;
            let repo = try_repo.unwrap();
            let file_path = format!("{}/{}", CREDENTIALS["path"], CREDENTIALS["file"]);
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .append(true)
                .open(file_path);
            let mut unwrapped = file.unwrap();
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
            info!("Commiting..");
            let commit_result = repo.commit(Option::Some("HEAD"),
                                            &git2::Signature::now(&CREDENTIALS["username"].clone(), &CREDENTIALS["email"].clone()).unwrap(),
                                            &git2::Signature::now(&CREDENTIALS["username"].clone(), &CREDENTIALS["email"].clone()).unwrap(),
                                            "test message",
                                            &tree,
                                            &[&parent_commit]);
            if commit_result.is_ok() {
                info!("Commited");
            } else {
                warn!("Not commited - {}", commit_result.err().unwrap().message());
            }

            let mut remote =  repo.find_remote("origin").unwrap();

            let mut po = PushOptions::new();
            let mut rco = RemoteCallbacks::new();
            rco.credentials(git_credentials_callback);

            po.remote_callbacks(rco);
            info!("Pushing..");
            let push_result = remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut po));
            if push_result.is_ok() {
                info!("Pushed");
            } else {
                warn!("Not pushed {}", push_result.err().unwrap().message());
            }
        }
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
    let key_file = CREDENTIALS["ssh_private_key"].clone();
    let key_file_pub = CREDENTIALS["ssh_pub_key"].clone();
    git2::Cred::ssh_key(
        user,
        Some(std::path::Path::new(&key_file_pub)),
        std::path::Path::new(&key_file),
         None,
    )
}
