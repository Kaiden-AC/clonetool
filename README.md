# clonetool

`clonetool` is a command-line utility written in Rust designed to clone all repositories for specified GitHub or GitLab users or organizations. It's a handy tool for quickly backing up or mirroring all of someone's repositories.

## Features

*   Clones all repositories for specified GitHub or GitLab users or organizations.
*   Clones repositories into separate folders named after the user/organization in a specified destination directory.
*   Uses `git clone --mirror` to create bare mirror repositories.
*   Provides a `--debug` flag for command output and more verbose messages.
*   Prompts the user before removing existing local directories with the same name as the repos to clone.
*   Allows automated "yes" responses for directory removal with the `--yes` or `-y` flag.
*   Allows automated "no" responses for directory removal with the `--no` or `-n` flag.
*   Allows excluding specific packages using the `--exclude` or `-x` flag.
*   Allows selecting between GitHub or GitLab using the `--provider` flag.

## How to Use

### Prerequisites

*   **Rust:** Make sure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org). (Cargo is included with the Rust installation.)
*   **Git:** Ensure that Git is installed and available in your system's `PATH`.

### Building the Tool

1.  Clone or download the source code for `clonetool`.
2.  Navigate to the root directory of the project (where `Cargo.toml` is located).
3.  Build the tool using Cargo:

    ```bash
    cargo build --release
    ```

    This command builds an optimized version of the tool that will be located in the `target/release` directory.

### Running the Tool

The basic usage of `clonetool` is:

```bash
clonetool -u <usernames> -d <destination_directory> [options]
```

*   `-u, --users <usernames>`: A comma-separated list of GitHub or GitLab usernames or organization names.
*   `-d, --destination <destination_directory>`: The local directory to clone repositories into.
*   `--debug`: Enables debug output, showing executed git commands.
*   `-y, --yes`: Automatically removes existing local directories without prompting.
*   `-n, --no`: Automatically skips cloning if a local directory exists.
*   `-x, --exclude <packages>`: Comma-separated list of packages to exclude, in the form of `user/package`.
*   `--provider <provider>`: The provider to use: `github` or `gitlab`. Default is `github`.

### Examples

1.  **Clone all repositories for a user from GitHub to the `/home/user/repos` directory:**

    ```bash
    clonetool -u testuser -d /home/user/repos
    ```
    This will create a directory called `testuser` inside the `repos` directory and all the repos inside that directory will be cloned into separate folders.

2.  **Clone all repositories for multiple users from GitHub to the `/mnt/data/github` directory:**

    ```bash
    clonetool -u testuser1,testuser2,testorg -d /mnt/data/github
    ```
    This will create a directory called `testuser1`, `testuser2` and `testorg` inside the `github` directory and all the repos inside each user/org directory will be cloned into separate folders.

3.  **Clone repositories with debugging output:**

    ```bash
    clonetool -u testuser -d /home/user/repos --debug
    ```

4.  **Clone repositories and automatically remove any existing directories:**

    ```bash
    clonetool -u testuser -d /home/user/repos --yes
    ```

5.  **Clone repositories and automatically skip any existing directories:**

    ```bash
    clonetool -u testuser -d /home/user/repos --no
    ```

6.  **Clone repositories, skipping some packages using the `--exclude` flag:**

    ```bash
    clonetool -u testuser -d /home/user/repos -x testuser/repo1,testuser/repo2
    ```
    This will clone all repos from `testuser`, except for the repos called `repo1` and `repo2`.

7.  **Clone all repositories for a user from GitLab to the `/home/user/repos` directory:**

    ```bash
    clonetool -u testuser -d /home/user/repos --provider gitlab
    ```
    This will clone all repositories in GitLab under the user named `testuser`.

8.  **Clone all repositories for multiple users from GitLab to the `/mnt/data/github` directory:**

    ```bash
    clonetool -u testuser1,testuser2,testorg -d /mnt/data/github --provider gitlab
    ```
    This will create a directory called `testuser1`, `testuser2` and `testorg` inside the `github` directory and all the repos inside each user/org directory will be cloned into separate folders.

### Note

*   The tool uses `git clone --mirror` so, while all branches are cloned, it does not clone a standard working directory.
*   There is unfortunately no way to pull changes in a bare repo for all branches at once. As a workaround, you can clone the repos again. The program will prompt you to remove existing local folders so you can clone the latest version.
*   If a user or organization deletes a repository, `clonetool` will not ask to remove that repository because it no longer gets the path from the GitHub or GitLab API.
*   The program will create the specified destination directory if it does not exist.
*   The program will create separate directories inside the destination, with each directory named after the user/org provided.
*   Usernames/organization names must only contain alphanumeric characters, underscores, or hyphens.
*   Destination directories do not have to be absolute paths.

## License

This project is open source and available under the [MIT License](LICENSE). The full text of the license can be found in the `LICENSE` file.

## Acknowledgment

This tool was created with the assistance of Google Gemini 2.0 Flash.
