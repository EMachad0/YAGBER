use std::process::Command;
use std::path::Path;

const TEST_ROMS_PATH: &str = "test_roms/";

const REPOS: [&str; 1] = [
    "https://github.com/retrio/gb-test-roms"
];

fn main() {
    println!("Cloning test roms!");
    
    for repo in REPOS {
        let repo_name = repo.split('/').last().unwrap();
        let path = format!("{}{}", TEST_ROMS_PATH, repo_name);
        if !Path::new(&path).exists() {
            println!("Cloning {} into {}", repo, path);
            Command::new("git")
                .arg("clone")
                .arg(repo)
                .arg(&path)
                .status()
                .expect("Failed to clone repository");
        } else {
            println!("Repository {} already exists at {}", repo, path);
        }
    }
}
