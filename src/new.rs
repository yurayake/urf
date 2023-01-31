use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::process::Command;

pub fn new(project_name: String) {
    // cargo new プロジェクト名 を実行
    let status = match Command::new("cargo")
        .args(&["new", project_name.as_str()])
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if status.success() != false {
        exit(1);
    }

    // Cargo.tomlを書き換える
    let tmp = format!("./{}/Cargo.toml", project_name);

    let status = match Command::new("rm").args(&[tmp.as_str()]).status() {
        Ok(status) => status,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if status.success() != false {
        exit(1);
    }

    let mut cargo_toml = match File::create(tmp.as_str()) {
        Ok(cargo_toml) => cargo_toml,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let cargo_toml_str = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]
log = \"0.4\"\nuefi = \"0.19\"\nuefi-services = \"0.16\"",
        project_name.as_str()
    );

    match cargo_toml.write_all(cargo_toml_str.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    // main.rsを書き換える
    let tmp = format!("./{}/src/main.rs", project_name);

    let status = match Command::new("rm").args(&[tmp.as_str()]).status() {
        Ok(status) => status,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if status.success() != false {
        exit(1);
    }

    let mut main_rs = match File::create(tmp.as_str()) {
        Ok(main_rs) => main_rs,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let main_rs_str = "#![no_main]\n#![no_std]\n#![feature(abi_efiapi)]\n#![allow(stable_features)]\n\nuse log::info;\nuse uefi::prelude::*;\n
#[entry]\nfn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {\n    uefi_services::init(&mut system_table).unwrap();
    info!(\"Hello world!\");\n    system_table.boot_services().stall(10_000_000);\n    Status::SUCCESS\n}\n";

    match main_rs.write_all(main_rs_str.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    // プロジェクトルートにrust-toolchein.tomlを作成し、設定を記入
    let tmp = format!("./{}/rust-toolchain.toml", project_name);

    let mut rust_toolchain_toml = match File::create(tmp.as_str()) {
        Ok(rust_toolchain_toml) => rust_toolchain_toml,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let rust_toolchain_toml_str =
        "[toolchain]\nchannel = \"nightly-2022-11-10\"\ntargets = [\"x86_64-unknown-uefi\"]\n";

    match rust_toolchain_toml.write_all(rust_toolchain_toml_str.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}
