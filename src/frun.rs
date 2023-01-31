use std::process::exit;
use std::process::Command;

pub fn frun() {
    // プロジェクトルートでcargo build --target x86_64-unknown-uefi を実行。
    let status = match Command::new("cargo")
        .args(&["build", "--target", "x86_64-unknown-uefi"])
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

    // cp /usr/share/OVMF/OVMF_CODE.fd . を実行
    let status = match Command::new("cp")
        .args(&["/usr/share/OVMF/OVMF_CODE.fd", "."])
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

    // cp /usr/share/OVMF/OVMF_VARS.fd . を実行
    let status = match Command::new("cp")
        .args(&["/usr/share/OVMF/OVMF_VARS.fd", "."])
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

    // mkdir -p esp/efi/boot を実行
    let status = match Command::new("mkdir").args(&["-p", "esp/efi/boot"]).status() {
        Ok(status) => status,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if status.success() != false {
        exit(1);
    }

    // cp ./target/x86_64-unknown-uefi/debug/プロジェクト名.efi ./esp/efi/boot/bootx64.efi を実行

    // プロジェクト名を取得する
    let current_dir_path = match std::env::current_dir() {
        Ok(current_dir_path) => current_dir_path,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let project_name = match current_dir_path.file_name() {
        None => {
            exit(1);
        }
        Some(project_name) => project_name,
    };

    let project_name = match project_name.to_str() {
        None => {
            exit(1);
        }
        Some(project_name) => project_name,
    };

    let tmp = format!("./target/x86_64-unknown-uefi/debug/{}.efi", project_name);

    let status = match Command::new("cp")
        .args(&[tmp.as_str(), "./esp/efi/boot/bootx64.efi"])
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

    /* qemu-system-x86_64 -enable-kvm \
         -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
         -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
         -drive format=raw,file=fat:rw:esp を実行
    */

    let status = match Command::new("qemu-system-x86_64")
        .args(&[
            "-drive",
            "if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd",
            "-drive",
            "if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd",
            "-drive",
            "format=raw,file=fat:rw:esp",
        ])
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
}
