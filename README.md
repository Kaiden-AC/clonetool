# clonetool

`clonetool` is a command-line utility written in Rust designed to clone all repositories for specified GitHub or GitLab users or organizations. It's a handy tool for quickly backing up or managing repositories, with added support for pulling updates to existing clones.

## Features

* Clones all repositories for specified GitHub or GitLab users or organizations.
* Clones repositories into separate folders named after the user/organization in a specified destination directory.
* Provides a `--debug` flag for verbose output, including executed git commands.
* Allows pulling updates (`git pull`) for all branches of existing repositories using the `--pull` or `-p` flag.
* Excludes specific repositories using the `--exclude` or `-x` flag.
* Supports GitHub and GitLab providers via the `--provider` flag.
* Automatically creates missing destination directories.

## How to Use

### Prerequisites

* **Rust:** Make sure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org). (Cargo is included with the Rust installation.)
* **Git:** Ensure that Git is installed and available in your system's `PATH`.

### Building the Tool

1. Clone or download the source code for `clonetool`.
2. Navigate to the root directory of the project (where `Cargo.toml` is located).
3. Build the tool using Cargo:

    ```bash
    cargo build --release
    ```

    This will create an optimized version of the tool located in the `target/release` directory.

### Running the Tool

The basic usage of `clonetool` is:

```bash
clonetool -u <usernames> -d <destination_directory> [options]
```

#### Key Arguments and Options:
* `-u, --users <usernames>`: A comma-separated list of GitHub or GitLab usernames or organization names.
* `-d, --destination <destination_directory>`: The local directory to clone repositories into.
* `--debug`: Enables debug output, showing executed git commands.
* `-p, --pull`: Pulls updates (`git pull`) for all branches of repositories in the specified destination directory.
* `-x, --exclude <packages>`: A comma-separated list of repositories to exclude, in the form `user/package`.
* `--provider <provider>`: Specifies the provider (`github` or `gitlab`). Defaults to `github`.

### Examples

1. **Clone all repositories for a user from GitHub to the `/home/user/repos` directory:**

    ```bash
    clonetool -u testuser -d /home/user/repos
    ```

    This will create a directory called `testuser` inside the `repos` directory and clone all repositories inside.

2. **Clone all repositories for multiple users from GitHub:**

    ```bash
    clonetool -u user1,user2,org -d /mnt/data/repos
    ```

    This will create directories `user1`, `user2`, and `org` inside `/mnt/data/repos`, with each user's repositories cloned inside their respective directories.

3. **Pull updates for existing repositories:**

    ```bash
    clonetool -u testuser -d /home/user/repos --pull
    ```

    This will traverse the `/home/user/repos/testuser` directory and run `git pull` for all branches of each repository.

4. **Clone repositories while excluding specific repositories:**

    ```bash
    clonetool -u testuser -d /home/user/repos -x testuser/repo1,testuser/repo2
    ```

    This will clone all repositories from `testuser`, except `repo1` and `repo2`.

5. **Clone repositories from GitLab:**

    ```bash
    clonetool -u testuser -d /home/user/repos --provider gitlab
    ```

    This will clone all repositories for `testuser` from GitLab.

6. **Pull updates for multiple users:**

    ```bash
    clonetool -u user1,user2,org -d /mnt/data/repos --pull
    ```

    This will pull updates for repositories inside `/mnt/data/repos/user1`, `/mnt/data/repos/user2`, and `/mnt/data/repos/org`.

### Notes

* When using `--pull`, the tool assumes the repositories already exist in the destination directory.
* The tool supports both GitHub and GitLab APIs for fetching repository lists.
* The program will create the specified destination directory if it does not exist.
* Usernames/organization names must only contain alphanumeric characters, underscores, or hyphens.
* The `--exclude` flag requires repository names in the format `user/repository`.

## License

This project is open source and available under the [MIT License](LICENSE). The full text of the license can be found in the `LICENSE` file.

## Acknowledgment

This tool was created with the assistance of ChatGPT 4o and Google Gemini 2.0 Flash.
