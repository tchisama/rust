use dialoguer::{Input, MultiSelect, Select};
use std::fs::read_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    // Ask for project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact_text()
        .unwrap();

    // Ask for Odoo version
    let odoo_versions = vec!["18", "17", "16", "15", "14", "13"];
    let odoo_version_index = Select::new()
        .with_prompt("Select Odoo version")
        .items(&odoo_versions)
        .interact()
        .unwrap();
    let odoo_version = odoo_versions[odoo_version_index];

    // Map Odoo version to PostgreSQL version
    let postgres_version = match odoo_version {
        "17" | "18" => "16",
        _ => "13",
    };

    // Create folder structure
    let folders = vec!["config", "addons", "custom_addons", "data"];
    for folder in folders {
        let path = format!("{}/{}", project_name, folder);
        fs::create_dir_all(&path).expect("Failed to create directory");
    }

    // Getting the port number

    let default_port = 8069;

    // Find an available port
    let available_port = find_available_port(default_port);
    println!("Using port: {}", available_port);

    // Generate docker-compose.yml
    let docker_compose_content = format!(
        r#"version: '3'
services:
  web:
    image: odoo:{}
    ports:
        - "{}:8069"
    volumes:
      - ./config:/etc/odoo
      - ./addons:/mnt/extra-addons
      - ./custom_addons:/mnt/custom-addons
      - ./data:/var/lib/odoo
    depends_on:
      - postgres
    environment:
      - HOST=db
      - USER=odoo
      - PASSWORD=odoo
    restart: always

  postgres:
    image: postgres:{}
    environment:
      - POSTGRES_DB=postgres
      - POSTGRES_PASSWORD=odoo
      - POSTGRES_USER=odoo
    volumes:
      - ./data/pgdata:/var/lib/postgresql/data
    restart: always

volumes:
  odoo-web-data:
  postgres-data:
"#,
        odoo_version, available_port, postgres_version
    );

    let docker_compose_path = format!("{}/docker-compose.yml", project_name);
    let mut file = File::create(&docker_compose_path).expect("Failed to create file");
    file.write_all(docker_compose_content.as_bytes())
        .expect("Failed to write to file");

    // Generate odoo.conf
    let odoo_conf_content = format!(
        r#"[options]
addons_path = /mnt/extra-addons,/mnt/custom-addons
data_dir = /var/lib/odoo
admin_passwd = admin
db_host = postgres
db_port = 5432
db_user = odoo
db_password = odoo
"#
    );

    let odoo_conf_path = format!("{}/config/odoo.conf", project_name);
    let mut odoo_conf_file = File::create(&odoo_conf_path).expect("Failed to create odoo.conf");
    odoo_conf_file
        .write_all(odoo_conf_content.as_bytes())
        .expect("Failed to write to odoo.conf");

    // Change permissions
    let permissions_commands = format!(
        r#"chmod -R 777 {}/addons
        chmod -R 777 {}/custom_addons
        chmod -R 777 {}/data"#,
        project_name, project_name, project_name
    );

    let _output = Command::new("sh")
        .arg("-c")
        .arg(permissions_commands)
        .status()
        .expect("Failed to execute command");

    println!("Permissions have been set successfully!");

    // check if you want to add addons from the repository
    let add_addons: bool = Select::new()
        .with_prompt("Do you want to add addons from the repository?")
        .default(0)
        .items(&["Yes", "No"])
        .interact()
        .unwrap()
        == 0;

    // if no, exit
    if add_addons {
        // Clone the repository if it doesn't exist
        let repo_path = format!(
            "{}/Documents/BBG-ODOO-ADDONS-PACK",
            std::env::var("HOME").unwrap()
        );
        if Path::new(&repo_path).exists() {
            println!("Addons already exists!");
            //  checkout the correct branch and pull the latest changes
            let status = Command::new("git")
                .arg("checkout")
                .arg(format!("{}.0", odoo_version))
                .current_dir(&repo_path)
                .status()
                .expect("Failed to execute git checkout");

            if status.success() {
                println!("Switched to branch: {}.0", odoo_version);
            } else {
                eprintln!("Failed to switch to branch: {}.0", odoo_version);
            }

            // chek if want to pull the latest changes
            let pull_latest: bool = Select::new()
                .with_prompt("Do you want to pull the latest changes?")
                .default(0)
                .items(&["Yes", "No"])
                .interact()
                .unwrap()
                == 0;
            if pull_latest {
                let status = Command::new("git")
                    .arg("pull")
                    .current_dir(&repo_path)
                    .status()
                    .expect("Failed to execute git pull");

                if status.success() {
                    println!("Repository updated successfully!");
                } else {
                    eprintln!("Failed to update repository!");
                }
            }
        } else {
            println!("Cloning Bigbang Odoo Addons Pack repository...");

            // Clone the repository
            let status = Command::new("git")
                .arg("clone")
                .arg("https://github.com/bbgstack/BBG-ODOO-ADDONS-PACK")
                .arg(&repo_path)
                .arg("--depth")
                .arg("1")
                .arg("--branch")
                .arg(format!("{}.0", odoo_version))
                .status()
                .expect("Failed to execute git clone");

            if status.success() {
                println!("Repository cloned successfully!");
            } else {
                eprintln!("Failed to clone repository!");
            }
        }

        // list all the addons
        // make an array of all the selected addons

        // List all addons in the repository
        let addons: Vec<String> = read_dir(&repo_path)
            .expect("Failed to read repository directory")
            .filter_map(|entry| {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.is_dir() {
                    Some(path.file_name().unwrap().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect();

        if addons.is_empty() {
            println!("No addons found in the repository!");
            return;
        }

        // Multi-select prompt for addons
        let selected_addons: Vec<usize> = MultiSelect::new()
            .with_prompt("Select addons to include in your project")
            .items(&addons)
            .interact()
            .expect("Failed to select addons");

        // Copy selected addons to custom_addons folder
        for index in selected_addons {
            let addon_name = &addons[index];
            let src = format!("{}/{}", repo_path, addon_name);
            let dest = format!("{}/custom_addons", project_name); // Destination is always custom_addons

            // Copy directory or group
            copy_dir_all(&src, &dest).expect("Failed to copy addon");
            println!("Copied addon: {}", addon_name);
        }
    }

    println!(
        "Project '{}' with Odoo {}  has been created successfully!",
        project_name, odoo_version
    );

    // check if you want to start docker compose
    let start_docker: bool = Select::new()
        .with_prompt("Do you want to start the docker-compose?")
        .default(0)
        .items(&["Yes", "No"])
        .interact()
        .unwrap()
        == 0;

    if start_docker {
        let _output = Command::new("docker")
            .arg("compose")
            .arg("up")
            .arg("-d")
            .current_dir(&project_name)
            .status()
            .expect("Failed to execute command");

        println!("Docker compose has been started successfully!");
    }
}

// Function to check if a port is in use
fn is_port_in_use(port: u16) -> bool {
    // Run the `ss` command to check if the port is in use
    let output = Command::new("ss")
        .arg("-tuln")
        .arg(format!("sport {}", port))
        .output();

    match output {
        Ok(output) => {
            // Convert the output to a string
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Check if the output contains the port
            output_str.contains(&format!(":{} ", port))
        }
        Err(_) => {
            // If the command fails (e.g., `ss` is not installed), assume the port is available
            println!(
                "Warning: 'ss' command failed. Assuming port {} is available.",
                port
            );
            false
        }
    }
}

// Function to find an available port
fn find_available_port(start_port: u16) -> u16 {
    let mut port = start_port;
    while is_port_in_use(port) {
        println!(
            "Port {} is already in use. Trying port {}...",
            port,
            port + 1
        );
        port += 1;
    }
    port
}

// Helper function to copy directories or groups
fn copy_dir_all(src: &str, dst: &str) -> std::io::Result<()> {
    // Ensure the destination directory exists
    fs::create_dir_all(dst)?;

    // Check if the source directory contains __manifest__.py
    let manifest_path = Path::new(src).join("__manifest__.py");

    if manifest_path.exists() {
        // If __manifest__.py exists, treat it as a single addon
        println!("Copying single addon: {}", src);

        let dest_path = format!(
            "{}/{}",
            dst,
            Path::new(src).file_name().unwrap().to_string_lossy()
        );
        // Copy the entire directory as-is to custom_addons
        copy_dir_contents(src, &dest_path)?;
    } else {
        // If __manifest__.py doesn't exist, treat it as a group
        println!("Treating {} as a group of addons", src);

        // Iterate over all immediate contents of the group
        for entry in fs::read_dir(src)? {
            let entry = entry?; // Handle potential errors in reading the entry
            let entry_path = entry.path(); // Get the full path of the entry
            let entry_name = entry.file_name().to_string_lossy().into_owned(); // Get the name of the entry

            // Create the destination path in custom_addons
            let dest_path = format!("{}/{}", dst, entry_name);

            // If it's a directory, copy it recursively
            if entry_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                copy_dir_contents(entry_path.to_str().unwrap(), &dest_path)?;
            } else {
                // If it's a file, copy it directly
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    }

    Ok(())
}

// Helper function to copy all contents of a directory
fn copy_dir_contents(src: &str, dst: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?; // Handle potential errors in reading the entry
        let entry_path = entry.path(); // Get the full path of the entry
        let entry_name = entry.file_name().to_string_lossy().into_owned(); // Get the name of the entry

        // Create the destination path
        let dest_path = format!("{}/{}", dst, entry_name);

        // If it's a directory, copy it recursively
        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_contents(entry_path.to_str().unwrap(), &dest_path)?;
        } else {
            // If it's a file, copy it directly
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}
