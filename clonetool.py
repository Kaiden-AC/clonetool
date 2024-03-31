import os
import subprocess
import configparser
import platform

CONFIG_FILE = "clonetool.cfg"

def clear_terminal():
    if platform.system() == 'Windows':
        os.system('cls')
    else:
        os.system('clear')

def display_menu():
    print("----------------\nclonetool v0.0.1\n----------------")
    print("1. Clone repositories")
    print("2. Pull changes from repositories")
    print("3. Add a new group")
    print("4. Exit")
    choice = input("Enter your choice: ")
    return choice

def add_new_group():
    group_name = input("Enter group name: ")
    output_dir = input("Enter output directory: ")
    repo_urls = input("Enter repository URLs separated by spaces: ").split()
    
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    config[group_name] = {'output_dir': output_dir, 'repo_urls': ' '.join(repo_urls)}
    
    with open(CONFIG_FILE, 'w') as configfile:
        config.write(configfile)
        clear_terminal()
    print("Group added successfully.")

def clone_repositories():
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    
    groups = list(config.sections())
    
    if not groups:
        clear_terminal()
        print("No groups found in the configuration file.")
        return
    
    clear_terminal()
    print("Available groups:")
    for i, group_name in enumerate(groups, start=1):
        print(f"{i}. {group_name}")
    
    group_choice = input("Enter the number of the group to clone (or '0' for all groups): ")
    try:
        if group_choice == '0':  # If '0' selected, clone all groups
            selected_groups = groups
        else:
            group_index = int(group_choice) - 1
            selected_groups = [groups[group_index]]
        
        for selected_group in selected_groups:
            output_dir = config.get(selected_group, 'output_dir', fallback=None)
            if not output_dir:
                print(f"Output directory not found for group '{selected_group}'.")
                continue
            
            repo_urls = config[selected_group]['repo_urls'].split()
            
            if not repo_urls:
                print(f"No repositories found for group '{selected_group}'.")
                continue
            
            os.makedirs(output_dir, exist_ok=True)
            os.chdir(output_dir)

            bare_choice = input(f"Do you want to clone '{selected_group}' as bare repositories? (yes/no): ").lower()
            bare_flag = bare_choice == 'yes'
            
            if bare_flag:
                for url in repo_urls:
                    subprocess.run(['git', 'clone', '--bare', url.strip()])
                clear_terminal()
                print(f"Bare repositories for group '{selected_group}' cloned successfully.")
            else:
                recursive_choice = input("Do you want to clone recursively? (yes/no): ").lower()
                recursive_flag = recursive_choice == 'yes'

                for url in repo_urls:
                    if recursive_flag:
                        subprocess.run(['git', 'clone', '--recursive', url.strip()])
                    else:
                        subprocess.run(['git', 'clone', url.strip()])
                
                clear_terminal()
                print(f"Repositories for group '{selected_group}' cloned successfully{' recursively' if recursive_flag else ''}.")
    except (ValueError, IndexError):
        print("Invalid group choice. Please enter a valid number or '0' for all groups.")

def pull_changes():
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    
    found_repos = False
    
    for group_name in config.sections():
        if group_name == 'Settings':  # Skip the 'Settings' section
            continue
        
        output_dir = config.get(group_name, 'output_dir', fallback=None)
        if not output_dir:
            print(f"Output directory not found for group '{group_name}'.")
            continue
        
        for repo_url in config[group_name]['repo_urls'].split():
            found_repos = True
            repo_name = repo_url.split('/')[-1].split('.')[0]  # Extracting repo name from URL
            repo_dir = os.path.join(output_dir, repo_name).replace("\\", "/")  # Replace backslashes with forward slashes
            
            if os.path.exists(repo_dir):
                print(f"Pulling changes for repository '{repo_name}' in directory '{repo_dir}'...")
                try:
                    # Execute 'git pull' command
                    git_pull_result = subprocess.run(['git', 'pull'], capture_output=True, text=True, cwd=repo_dir)
                    
                    # Check if there's any error
                    if git_pull_result.returncode != 0:
                        print(f"Error pulling changes for repository '{repo_name}': {git_pull_result.stderr}")
                        continue
                    
                    # Execute 'git submodule update --recursive' command
                    git_submodule_result = subprocess.run(['git', 'submodule', 'update', '--recursive'], capture_output=True, text=True, cwd=repo_dir)
                    
                    # Check if there's any error
                    if git_submodule_result.returncode != 0:
                        print(f"Error updating submodules for repository '{repo_name}': {git_submodule_result.stderr}")
                except Exception as e:
                    print(f"Error pulling changes for repository '{repo_name}': {e}")
            else:
                print(f"Repository directory '{repo_dir}' not found.")
    
    if not found_repos:
        print("No repositories found in the configuration file.")
    else:
        # After finishing all tasks, clear the terminal
        clear_terminal()
        print("Changes pulled successfully.")

def main():
    clear_terminal()
    
    while True:
        choice = display_menu()
        
        if choice == '1':
            clone_repositories()
        elif choice == '2':
            pull_changes()
        elif choice == '3':
            add_new_group()
        elif choice == '4':
            clear_terminal()
            print("Exiting...")
            break
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()
