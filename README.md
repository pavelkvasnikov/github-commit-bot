# github-commit-bot

Simple bot which pushes commits to github adding a string to a specific file each 3.5 secs

# instalattion
1) install Rust from https://rustup.rs/
2) install git 
3) clone this repo
4) run `cargo build --release`
5) run binary executable in `target/release/github-commit-bot`

# Usage

Example run
```
nohup ./github-commit-bot repo=git@github.com:username/reponame.git path=local_dir_path ssh_pub_key=/home/username/.ssh/id_rsa.pub ssh_private_key=/home/username/.ssh/id_rsa username=username email=example@example.com file=file_in_repo_to_commit > /dev/null &

```


# Additions
bot writes to `log/default_log.log` file only, no stdout/stderr
if you need to change timeout it's in `main.rs:14 TIMEOUT`

# FAQ
No FAQ, ask qustions and request features vie issues

