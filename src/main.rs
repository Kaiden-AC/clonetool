use clap::{App, Arg};
use regex::Regex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn clone_or_pull_repositories(
    usernames: &Vec<String>,
    dest_dir: &Path,
    debug: bool,
    pull: bool,
    exclude: &HashSet<String>,
    provider: &str,
) -> Result<(), Error> {
    let client = Client::new();

    for username in usernames {
        let user_dir = dest_dir.join(username);

        if !user_dir.exists() {
            if debug {
                println!("Creating user directory: {}", user_dir.display());
            }
            fs::create_dir_all(&user_dir)?;
        }

        let api_url = match provider {
            "github" => format!("https://api.github.com/users/{}/repos?per_page=1000", username),
            "gitlab" => format!("https://gitlab.com/api/v4/users/{}/projects?per_page=1000", username),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid provider specified. Use 'github' or 'gitlab'.",
                ))
            }
        };

        let response = client
        .get(&api_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .send();

        match response {
            Ok(res) if res.status().is_success() => {
                let repos: Vec<Value> = res.json().unwrap_or_default();

                for repo in repos {
                    let (repo_name, clone_url) = match provider {
                        "github" => (
                            repo["name"].as_str().unwrap_or(""),
                                     repo["clone_url"].as_str().unwrap_or(""),
                        ),
                        "gitlab" => (
                            repo["name"].as_str().unwrap_or(""),
                                     repo["http_url_to_repo"].as_str().unwrap_or(""),
                        ),
                        _ => ("", ""),
                    };

                    if repo_name.is_empty() || clone_url.is_empty() {
                        eprintln!("Invalid repo data for user: {}. Skipping...", username);
                        continue;
                    }

                    let full_repo_name = format!("{}/{}", username, repo_name);
                    if exclude.contains(&full_repo_name) {
                        if debug {
                            println!("Skipping excluded repository: {}", full_repo_name);
                        }
                        continue;
                    }

                    let repo_path = user_dir.join(repo_name);

                    if repo_path.exists() {
                        if pull {
                            println!("Pulling changes in repository: {}", repo_path.display());
                            git_pull(&repo_path, debug)?;
                        } else {
                            println!(
                                "Repository already exists, skipping clone: {}",
                                repo_path.display()
                            );
                        }
                        continue;
                    }

                    println!("Cloning repository: {} into {}", repo_name, repo_path.display());
                    git_clone(clone_url, &repo_path, debug)?;
                }
            }
            Ok(res) => eprintln!(
                "Failed to fetch repositories for user {}: HTTP {}",
                username,
                res.status()
            ),
            Err(e) => eprintln!("Error fetching repositories for {}: {}", username, e),
        }
    }
    Ok(())
}

fn git_pull(repo_path: &Path, debug: bool) -> Result<(), Error> {
    let mut pull_command = Command::new("git");
    pull_command.current_dir(repo_path).arg("pull").arg("--all");

    if debug {
        println!("Executing: {:?}", pull_command);
    }

    let status = pull_command.status()?;
    if !status.success() {
        eprintln!(
            "Failed to pull changes in repository: {}. Status: {:?}",
            repo_path.display(),
                  status
        );
    } else if debug {
        println!("Successfully pulled changes in {}", repo_path.display());
    }
    Ok(())
}

fn git_clone(clone_url: &str, repo_path: &Path, debug: bool) -> Result<(), Error> {
    let mut clone_command = Command::new("git");
    clone_command.arg("clone").arg(clone_url).arg(repo_path);

    if debug {
        println!("Executing: {:?}", clone_command);
    }

    let status = clone_command.status()?;
    if !status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Failed to clone repository: {}", clone_url),
        ));
    }
    if debug {
        println!("Successfully cloned {}", clone_url);
    }
    Ok(())
}

fn validate_path(path_str: &str) -> Result<(), Error> {
    let path = Path::new(path_str);
    if !path.is_absolute() {
        return Err(Error::new(ErrorKind::InvalidInput, "Path is not absolute"));
    }
    Ok(())
}

fn validate_username(username: &str) -> Result<(), Error> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !re.is_match(username) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid username format. Only letters, numbers, underscores, and dashes are allowed.",
        ));
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = App::new("clonetool")
    .version("0.2")
    .about("Clones or pulls repositories for specified GitHub or GitLab users.")
    .arg(
        Arg::with_name("users")
        .short("u")
        .long("users")
        .value_name("USERNAMES")
        .help("Comma-separated list of GitHub/GitLab usernames or organizations")
        .takes_value(true)
        .required(true),
    )
    .arg(
        Arg::with_name("destination")
        .short("d")
        .long("destination")
        .value_name("DEST_DIR")
        .help("Destination directory to clone repositories to")
        .takes_value(true)
        .required(true),
    )
    .arg(
        Arg::with_name("debug")
        .long("debug")
        .help("Enable debug output"),
    )
    .arg(
        Arg::with_name("pull")
        .short("p")
        .long("pull")
        .help("Pull changes for all repositories in the destination directory"),
    )
    .arg(
        Arg::with_name("exclude")
        .short("x")
        .long("exclude")
        .value_name("EXCLUDE_REPOS")
        .help("Comma-separated list of repos to exclude in the form user/repo")
        .takes_value(true),
    )
    .arg(
        Arg::with_name("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("Provider to use: github or gitlab (default: github)")
        .takes_value(true)
        .default_value("github"),
    )
    .get_matches();

    let debug = matches.is_present("debug");
    let pull = matches.is_present("pull");
    let provider = matches.value_of("provider").unwrap();

    let exclude_str = matches.value_of("exclude").unwrap_or("");
    let exclude: HashSet<String> = exclude_str.split(',').map(|s| s.trim().to_string()).collect();

    let usernames_str = matches.value_of("users").expect("Users argument is required");
    let usernames: Vec<String> = usernames_str.split(',').map(|s| s.trim().to_string()).collect();

    for username in &usernames {
        validate_username(username)?;
    }

    let dest_dir_str = matches.value_of("destination").expect("Destination argument is required");
    validate_path(dest_dir_str)?;
    let dest_dir = Path::new(dest_dir_str);

    clone_or_pull_repositories(&usernames, dest_dir, debug, pull, &exclude, provider)?;

    Ok(())
}
