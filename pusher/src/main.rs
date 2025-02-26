use git2::{Repository, StatusOptions};
use dialoguer::{MultiSelect, Input};
use serde::{Serialize, Deserialize};
use std::{fs, io::Write, path::Path, process::Command};

#[derive(Serialize, Deserialize)]
struct Config {
    username: String,
    token: String,
}

fn main() {
    let config = get_or_create_config();
    let file_path = "projects.txt";

    let project_paths = match fs::read_to_string(file_path) {
        Ok(content) => content.lines().map(String::from).collect::<Vec<_>>(),
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return;
        }
    };

    let mut projects_with_changes = Vec::new();
    
    for path in &project_paths {
        if let Ok(repo) = Repository::open(path) {
            let mut status_opts = StatusOptions::new();
            status_opts.include_untracked(true);
            
            if let Ok(statuses) = repo.statuses(Some(&mut status_opts)) {
                if !statuses.is_empty() {
                    projects_with_changes.push(path.clone());
                }
            }
        }
    }
    
    if projects_with_changes.is_empty() {
        println!("No projects have uncommitted changes.");
        return;
    }
    
    let selections = MultiSelect::new()
        .with_prompt("Select projects to commit & push")
        .items(&projects_with_changes)
        .interact()
        .unwrap();
    
    for &index in &selections {
        let project = &projects_with_changes[index];
        println!("Processing project: {}", project);
        
        let commit_message: String = Input::new()
            .with_prompt(format!("Enter commit message for {}", project))
            .interact_text()
            .unwrap();
        
        if let Err(e) = run_git_commands(project, &commit_message, &config) {
            eprintln!("Failed to process {}: {}", project, e);
        }
    }
}

fn get_or_create_config() -> Config {
    let config_path = "config.json";

    if Path::new(config_path).exists() {
        let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
        serde_json::from_str(&config_content).expect("Failed to parse config file")
    } else {
        let username: String = Input::new()
            .with_prompt("Enter your GitHub username")
            .interact_text()
            .unwrap();

        let token: String = Input::new()
            .with_prompt("Enter your GitHub personal access token")
            .interact_text()
            .unwrap();

        let config = Config { username, token };

        let json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
        let mut file = fs::File::create(config_path).expect("Failed to create config file");
        file.write_all(json.as_bytes()).expect("Failed to write to config file");

        println!("GitHub credentials saved to config.json.");

        config
    }
}

fn run_git_commands(project: &str, message: &str, config: &Config) -> Result<(), String> {
    let path = Path::new(project);

    let commands = vec![
        ("git", vec!["add", "."]),
        ("git", vec!["commit", "-m", message]),
        ("git", vec!["push", "https://", &config.username, ":", &config.token, "@github.com/your-repo.git"]),
    ];

    for (cmd, args) in commands {
        let output = Command::new(cmd)
            .args(&args)
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to run {}: {}", cmd, e))?;

        if !output.status.success() {
            return Err(format!("{} failed: {}", cmd, String::from_utf8_lossy(&output.stderr)));
        }
    }

    Ok(())
}

