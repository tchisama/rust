use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::process::{Command, exit};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: tailwind <component-url>");
        exit(1);
    }

    let component_url = &args[1];
    let component_name = extract_component_name(component_url);
    let base_dir = "/home/tchisama/github/rust/tailwind-components/src/tailwind-components/"

    let component_dir = base_dir.join("components").join("react").join("components")
        .join("application-ui").join(&component_name);
    
    if !component_dir.exists() {
        eprintln!("Directory not found: {:?}", component_dir);
        exit(1);
    }

    let files = fs::read_dir(component_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    if files.is_empty() {
        eprintln!("No files found in the directory.");
        exit(1);
    }

    // Display the files in the directory
    println!("Available components:");
    for (i, file) in files.iter().enumerate() {
        println!("{}) {}", i + 1, file);
    }

    // Get the user input to choose a file
    let mut choice = String::new();
    print!("Enter the number of the file you want to copy to clipboard: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut choice)?;

    let choice: usize = choice.trim().parse().unwrap_or(0);
    if choice == 0 || choice > files.len() {
        eprintln!("Invalid choice.");
        exit(1);
    }

    let selected_file = &files[choice - 1];

    // Read the selected file's content
    let selected_file_path = component_dir.join(selected_file);
    let contents = fs::read_to_string(selected_file_path)?;

    // Copy to clipboard
    let mut clipboard = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    clipboard.stdin.as_mut().unwrap().write_all(contents.as_bytes())?;

    println!("The contents of {} have been copied to the clipboard.", selected_file);

    Ok(())
}

fn extract_component_name(url: &str) -> String {
    // For example, extract `sidebar-navigation` from the URL
    let parts: Vec<&str> = url.split('/').collect();
    parts.last().unwrap_or(&"").to_string()
}

fn get_executable_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf()
}








// use std::env;
// use std::fs::{self, File};
// use std::io::{self, Write, Read}; // <-- Add this import for Read trait
// use std::process::Command;
//
// fn main() -> io::Result<()> {
//     // Define the base directory where Tailwind components are stored
//     let base_dir = format!("{}/src/tailwind-components/components/react/components", env::var("PWD").unwrap());
//
//
//     // Get the URL argument
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         println!("Usage: cargo run <Tailwind UI Component URL>");
//         return Ok(());
//     }
//
//     let url = &args[1];
//     let url_path = url.replace("https://tailwindui.com/components/", "");
//
//     // Construct the full target directory path
//     let target_dir = format!("{}/{}", base_dir, url_path);
//
//     // Check if the target directory exists
//     if !fs::metadata(&target_dir).is_ok() {
//         println!("Error: Directory '{}' not found.", target_dir);
//         return Ok(());
//     }
//
//     // List available components (just filenames, not full paths)
//     let files = fs::read_dir(&target_dir)?
//         .filter_map(|entry| entry.ok())
//         .map(|entry| entry.file_name().to_string_lossy().into_owned())
//         .collect::<Vec<String>>();
//
//     if files.is_empty() {
//         println!("No components found in '{}'.", target_dir);
//         return Ok(());
//     }
//
//     // Display file names and allow user to select one
//     println!("Available components in '{}':", target_dir
//         // split with / and get the last element
//         .split('/').last().unwrap()
//     );
//     for (i, file) in files.iter().enumerate() {
//         println!("{}: {}", i + 1, file);
//     }
//
//     let mut choice = String::new();
//     println!("Enter the number of the file you want to copy to clipboard:");
//     io::stdin().read_line(&mut choice)?;
//     let choice: usize = choice.trim().parse().unwrap_or(0);
//
//     if choice == 0 || choice > files.len() {
//         println!("Invalid selection.");
//         return Ok(());
//     }
//
//     let selected_file = &files[choice - 1];
//     let selected_file_path = format!("{}/{}", target_dir, selected_file);
//
//     // Read the contents of the selected file
//     let mut file = File::open(&selected_file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?; // <-- Now `read_to_string` should work
//
//     // Copy contents to clipboard
//     if let Err(_) = Command::new("xclip").arg("-selection").arg("clipboard").arg("-i").stdin(std::process::Stdio::piped()).spawn() {
//         // If xclip isn't available, try pbcopy (macOS)
//         if let Err(_) = Command::new("pbcopy").stdin(std::process::Stdio::piped()).spawn() {
//             println!("Error: No clipboard utility found. Install xclip (Linux) or use pbcopy (macOS).");
//             return Ok(());
//         }
//     }
//
//     // Write contents to clipboard
//     let mut clipboard = Command::new("xclip")
//         .arg("-selection")
//         .arg("clipboard")
//         .stdin(std::process::Stdio::piped())
//         .spawn()
//         .expect("Failed to copy to clipboard.");
//
//     clipboard.stdin.as_mut().unwrap().write_all(contents.as_bytes())?;
//
//     println!("Component copied to clipboard!");
//
//     Ok(())
// }
//
