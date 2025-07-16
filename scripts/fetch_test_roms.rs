#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! chrono = "0.4.41"
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_json = "1.0.140"
//! ```

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

const METADATA_VERSION: &str = "1.0.0";

const TEST_ROMS_PATH: &str = "test_roms/";

#[derive(Serialize, Deserialize)]
struct TestRomMetadata {
    source: String,
    version: String,
    date: String,
    metadata_version: String,
}

fn main() {
    println!("Cloning test roms!");

    // Make sure the base test_roms directory exists
    std::fs::create_dir_all(TEST_ROMS_PATH).expect("Failed to create test_roms directory");

    fetch_blargg_test_roms();
    fetch_mooneye_test_roms();
    fetch_dmg_acid2_test_roms();
    fetch_cgb_acid2_test_roms();
}

fn fetch_blargg_test_roms() {
    let repo = "https://github.com/retrio/gb-test-roms";
    let path = format!("{}{}", TEST_ROMS_PATH, "blargg/");
    if Path::new(&path).exists() {
        println!("Blargg test ROMs already exist at {}, SKIPPING", path);
        return;
    }

    println!("Cloning {} into {}", repo, path);
    Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(&path)
        .status()
        .expect("Failed to clone repository");

    // Retrieve the current commit hash so we can record the exact version.
    let commit_hash = get_commit_hash(&path);
    write_metadata(&path, "blargg", &commit_hash);

    println!(
        "Blargg test ROMs (version {}) fetched successfully to {}",
        commit_hash, path
    );
}

fn fetch_mooneye_test_roms() {
    // Lock to a specific pre-built Mooneye Test Suite version. Update this when you want to refresh.
    const MOONEYE_VERSION: &str = "mts-20240926-1737-443f6e1";

    let base_url = "https://gekkio.fi/files/mooneye-test-suite/";
    let dest_dir = format!("{}{}", TEST_ROMS_PATH, "mts/");

    // Skip if we have already fetched the archive.
    if Path::new(&dest_dir).exists() {
        println!("Mooneye test ROMs already exist at {}, SKIPPING", dest_dir);
        return;
    }

    // Compose FULL URL for the locked version.
    let zip_name = format!("{}.zip", MOONEYE_VERSION);
    let zip_url = format!("{}{}/{}", base_url, MOONEYE_VERSION, zip_name);
    let zip_local_path = format!("{}{}", TEST_ROMS_PATH, &zip_name);

    println!(
        "Downloading locked Mooneye build {} -> {}",
        MOONEYE_VERSION, zip_local_path
    );

    let status = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&zip_local_path)
        .arg(&zip_url)
        .status()
        .expect("Failed to download Mooneye test ROM archive");

    if !status.success() {
        panic!("curl returned non-zero exit status while downloading Mooneye archive");
    }

    // Extract the archive into the test_roms directory
    println!("Extracting {}", zip_local_path);
    let status = Command::new("unzip")
        .arg("-q")
        .arg(&zip_local_path)
        .arg("-d")
        .arg(TEST_ROMS_PATH)
        .status()
        .expect("Failed to unzip Mooneye archive (ensure 'unzip' is installed)");

    if !status.success() {
        panic!("unzip returned non-zero exit status while extracting Mooneye archive");
    }

    // Rename extracted directory to the stable destination (test_roms/mts)
    let extracted_path = format!("{}{}", TEST_ROMS_PATH, MOONEYE_VERSION);
    if Path::new(&extracted_path).exists() {
        std::fs::rename(&extracted_path, &dest_dir)
            .expect("Failed to rename extracted Mooneye directory");
    }

    // Clean up the downloaded zip file
    std::fs::remove_file(&zip_local_path).ok();

    // Record the metadata.
    write_metadata(&dest_dir, "mts", MOONEYE_VERSION);

    println!(
        "Mooneye test ROMs (version {}) fetched successfully to {}",
        MOONEYE_VERSION, dest_dir
    );
}

fn fetch_dmg_acid2_test_roms() {
    const DMG_ACID2_VERSION: &str = "v1.0";

    let repo = "https://github.com/mattcurrie/dmg-acid2";
    let path = format!("{}{}", TEST_ROMS_PATH, "dmg-acid2/");
    if Path::new(&path).exists() {
        println!("DMG Acid2 test ROMs already exist at {}, SKIPPING", path);
        return;
    }

    println!("Cloning {} into {}", repo, path);
    Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(&path)
        .status()
        .expect("Failed to clone repository");

    // Checkout the specific version
    Command::new("git")
        .arg("-C")
        .arg(&path)
        .arg("checkout")
        .arg(DMG_ACID2_VERSION)
        .arg("--detach")
        .status()
        .expect("Failed to checkout specific version");

    let rom_url = format!(
        "{}/releases/download/{}/dmg-acid2.gb",
        repo, DMG_ACID2_VERSION
    );
    let rom_local_path = format!("{}{}", path, "dmg-acid2.gb");

    println!("Downloading dmg-acid2 test ROM");
    Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&rom_local_path)
        .arg(&rom_url)
        .status()
        .expect("Failed to download dmg-acid2 test ROM");

    // Record the metadata.
    write_metadata(&path, "dmg-acid2", DMG_ACID2_VERSION);

    println!(
        "DMG Acid2 test ROM (version {}) fetched successfully to {}",
        DMG_ACID2_VERSION, path
    );
}

fn fetch_cgb_acid2_test_roms() {
    const CGB_ACID2_VERSION: &str = "v1.0";

    let repo = "https://github.com/mattcurrie/cgb-acid2";
    let path = format!("{}{}", TEST_ROMS_PATH, "cgb-acid2/");
    if Path::new(&path).exists() {
        println!("CGB Acid2 test ROMs already exist at {}, SKIPPING", path);
        return;
    }

    println!("Cloning {} into {}", repo, path);
    Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(&path)
        .status()
        .expect("Failed to clone repository");

    // Checkout the specific version
    Command::new("git")
        .arg("-C")
        .arg(&path)
        .arg("checkout")
        .arg(CGB_ACID2_VERSION)
        .arg("--detach")
        .status()
        .expect("Failed to checkout specific version");

    let rom_url = format!(
        "{}/releases/download/{}/cgb-acid2.gbc",
        repo, CGB_ACID2_VERSION
    );
    let rom_local_path = format!("{}{}", path, "cgb-acid2.gbc");

    println!("Downloading cgb-acid2 test ROM");
    Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&rom_local_path)
        .arg(&rom_url)
        .status()
        .expect("Failed to download cgb-acid2 test ROM");

    // Record the metadata.
    write_metadata(&path, "cgb-acid2", CGB_ACID2_VERSION);

    println!(
        "CGB Acid2 test ROM (version {}) fetched successfully to {}",
        CGB_ACID2_VERSION, path
    );
}

fn get_commit_hash(path: &str) -> String {
    let git_rev_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to get commit hash");

    String::from_utf8_lossy(&git_rev_output.stdout)
        .trim()
        .to_string()
}

fn write_metadata(path: &str, source: &str, version: &str) {
    let metadata = TestRomMetadata {
        source: source.to_string(),
        version: version.to_string(),
        date: chrono::Utc::now().to_string(),
        metadata_version: METADATA_VERSION.to_string(),
    };
    let json_path = format!("{}{}", path, "version.json");
    std::fs::write(json_path, serde_json::to_string(&metadata).unwrap())
        .expect("Failed to write version.json file");
}
