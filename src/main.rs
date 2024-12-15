use clap::{Arg, App};
use std::process::Command;
use std::path::Path;
use std::io::{Error, ErrorKind, Write};
use std::fs;
use regex::Regex;

fn clone_repositories(usernames: &Vec<String>, dest_dir: &Path, debug: bool, yes: bool, no: bool) -> Result<(), Error> {
    for username in usernames {
        let user_dir = dest_dir.join(username);

        if !user_dir.exists() {
             if debug { println!("Creating user directory: {}", user_dir.display()); }
             fs::create_dir_all(&user_dir)?;
        }


        let api_url = format!("https://api.github.com/users/{}/repos?per_page=1000", username); // max 100 per page
        let client = reqwest::blocking::Client::new();
        let response = client.get(&api_url).header("User-Agent", "Rust GitHub Cloner").send();
         match response {
             Ok(res) => {

                   if !res.status().is_success() {
                    eprintln!("API request failed for user {}: Status code {}", username, res.status());
                    continue; //skip the user
                  }
                  let repos: Result<Vec<serde_json::Value>, reqwest::Error> = res.json();
                  match repos {
                        Ok(repos_json) => {
                           if repos_json.is_empty() {
                             println!("No repositories found for user: {}", username);
                             continue; //skip the user
                           }


                            for repo in repos_json {
                                    let repo_name = repo["name"].as_str().unwrap_or("");
                                    let clone_url = repo["clone_url"].as_str().unwrap_or("");

                                    if repo_name.is_empty() || clone_url.is_empty() {
                                        eprintln!("Invalid repo data encountered for user: {}. Skipping...", username);
                                        continue; // Skip invalid repo data
                                    }

                                    let repo_path = user_dir.join(repo_name);
                                     if repo_path.exists() {
                                        if yes {
                                           if debug { println!("Removing existing directory: {} (automated)", repo_path.display())};
                                             fs::remove_dir_all(&repo_path)?;
                                         }
                                        else if no {
                                           println!("Skipping cloning of {} due to existing directory (automated)",repo_path.display());
                                           continue; // skip to next repo if the user doesnt remove the directory
                                         }
                                        else {
                                            print!("Directory {} already exists. Remove it and its contents? (y/N): ", repo_path.display());
                                            std::io::stdout().flush().expect("Failed to flush stdout");


                                          let mut input = String::new();
                                         std::io::stdin().read_line(&mut input).expect("Failed to read line");
                                          let input = input.trim().to_lowercase();

                                          if input == "y" {
                                               if debug { println!("Removing existing directory: {}", repo_path.display())};
                                                fs::remove_dir_all(&repo_path)?;


                                         } else{
                                                println!("Skipping cloning of {} due to existing directory",repo_path.display());
                                               continue; // skip to next repo if the user doesnt remove the directory
                                          }
                                      }
                                 }


                                     let mut clone_command = Command::new("git");
                                      clone_command.arg("clone")
                                         .arg("--mirror")
                                         .arg(clone_url)
                                         .arg(&repo_path);

                                    if debug {
                                          println!("+ {:?}", clone_command); // print git command
                                    }

                                     let status = clone_command.status()?;


                                   if !status.success(){
                                        eprintln!("Failed to clone repo {} for user: {}. Error Code: {}",repo_name,username,status);
                                   } else if debug { println!("Cloned repo {} successfully", repo_name); }
                           }
                        },
                        Err(e) => {
                             eprintln!("Failed to parse JSON for user {}: {}", username, e);
                        }
                   }

            },
             Err(e) => {
                  eprintln!("API request failed for user {}: {}", username, e);
             }
         }
     }
    Ok(())
}





fn validate_path(path_str: &str) -> Result<(), Error>{
    let path = Path::new(path_str);
    if !path.is_absolute(){
        return Err(Error::new(ErrorKind::InvalidInput, "Path is not absolute"));
    }

    Ok(())
}


fn validate_username(username: &str) -> Result<(),Error>{
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !re.is_match(username){
         return Err(Error::new(ErrorKind::InvalidInput, "Invalid username format"));
    }
    Ok(())
}



fn main() -> Result<(), Error> {
   let matches = App::new("clonetool")
        .version("0.1")
        .about("Clones all repositories for specified GitHub users/organizations.")
        .arg(Arg::with_name("users")
            .short("u")
            .long("users")
            .value_name("USERNAMES")
            .help("Comma separated list of GitHub usernames or organizations")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("destination")
            .short("d")
            .long("destination")
            .value_name("DEST_DIR")
            .help("Destination directory to clone repositories to")
            .takes_value(true)
            .required(true)
        )
       .arg(Arg::with_name("debug")
            .long("debug")
            .help("Enable debug output")
        )
       .arg(Arg::with_name("yes")
             .short("y")
             .long("yes")
             .help("Automatically remove existing directories, use with caution")
         )
       .arg(Arg::with_name("no")
             .short("n")
             .long("no")
             .help("Automatically skip cloning of existing directories")
         )

        .get_matches();


    let debug = matches.is_present("debug");
     let yes = matches.is_present("yes");
     let no = matches.is_present("no");

    let usernames_str = matches.value_of("users").expect("Users argument is required");
    let usernames: Vec<String> = usernames_str.split(',').map(|s| s.trim().to_string()).collect();

     for username in &usernames {
        validate_username(username)?;
     }

     let dest_dir_str = matches.value_of("destination").expect("Destination argument is required");
    validate_path(dest_dir_str)?;
    let dest_dir = Path::new(dest_dir_str);

     clone_repositories(&usernames, dest_dir, debug, yes, no)?;
     Ok(())

}
