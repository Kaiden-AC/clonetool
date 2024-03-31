# clonetool

clonetool is a Python script designed to streamline and automate common Git repository operations. Whether you're managing a single repository or multiple repositories across different projects, this tool offers a convenient way to clone repositories, pull changes, and organize them into groups.

## Features

- **Clone Repositories:** Quickly clone multiple repositories from specified URLs into designated output directories.
- **Pull Changes:** Easily pull the latest changes and update submodules from remote repositories, keeping your local copies up to date.
- **Group Organization:** Organize repositories into groups for easier management and operation.
- **Simple Configuration:** Configure repository groups, output directories, and repository URLs using a simple configuration file (`clonetool.cfg`), or just use the built-in group creation tool.
- **User-Friendly Interface:** Interactive menu-based interface for intuitive operation and navigation.

## Getting Started

1. **Clone the Repository:** Clone this repository to your local machine.
   ```
   git clone https://github.com/Kaiden-AC/clonetool.git
   ```

2. **Install Dependencies:** Ensure you have Python installed on your system. Additionally, make sure to have Git installed and configured.

4. **Run the Script:** Execute the `clonetool.py` script to start using the tool.
   ```
   python clonetool.py
   ```

5. **Navigate the Menu:** Use the interactive menu to perform operations such as cloning repositories, pulling changes, adding new groups, and exiting the tool.

## Requirements

- Python 3.x
- Git

## Example Configuration (clonetool.cfg)

```ini
[Group1]
output_dir = /path/to/output/directory1
repo_urls = https://github.com/user1/repo1 https://github.com/user2/repo2

[Group2]
output_dir = /path/to/output/directory2
repo_urls = https://github.com/user3/repo3 https://github.com/user4/repo4
```
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
