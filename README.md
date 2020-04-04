# github-commit-bot

Simple bot which pushes commits to github adding a string to a specific file


# Instalattion from source
1) install Rust from https://rustup.rs/
2) install git 
3) clone this repo
4) run `cargo build --release`
5) run binary executable in `target/release/github-commit-bot`

# Download binary
https://github.com/pavelkvasnikov/github-commit-bot/releases currently available only for Linux 

# Usage

Example run
```
nohup ./github-commit-bot repo=git@github.com:username/reponame.git path=local_dir_path ssh_pub_key=/home/username/.ssh/id_rsa.pub ssh_private_key=/home/username/.ssh/id_rsa username=username email=example@example.com file=file_in_repo_to_commit timeout=integer_in_milliseconds > /dev/null &

```


# Additions
bot writes to `log/default_log.log` file only, no stdout/stderr

# FAQ
No FAQ, ask qustions and request features via issues
