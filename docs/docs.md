# 使い方
### urf --new プロジェクト名
新しいcargoプロジェクトを作成し、uefi-rsをセットアップします。
具体的には下記のことが行われます。<br>
1, cargo new プロジェクト名 を実行します。<br>
2, Cargo.tomlを下記の内容に書き換えます。
```
[package]
name = "プロジェクト名"
version = "0.1.0"
edition = "2021"

[dependencies]
log = "0.4"
uefi = "0.19"
uefi-services = "0.16"
```
3, main.rsの中身を書き換えます。

```
#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
```
4, プロジェクトルートにrust-toolchain.tomlを作成し、下記を記入します。
```
[toolchain]
channel = "nightly-2022-11-10"
targets = ["x86_64-unknown-uefi"]
```

### urf --frun
プロジェクトをビルドし、qemu上で実行します。
なおこのコマンドはプロジェクトルートで行ってください。
また初めてのビルド&実行時以外に使用しないでください。
具体的には下記のことが行われます。<br>
1, cargo build --target x86_64-unknown-uefi を実行します。<br>
2, 下記を実行します。
```
cp /usr/share/OVMF/OVMF_CODE.fd .
cp /usr/share/OVMF/OVMF_VARS.fd .
```
3, 下記を実行します。
```
mkdir -p esp/efi/boot
cp ./target/x86_64-unknown-uefi/debug/プロジェクト名.efi ./esp/efi/boot/bootx64.efi
```
4, 下記を実行します。
```
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
```

### urf --rrun
プロジェクトを再ビルドし、qemu上で実行します。
なおこのコマンドはプロジェクトルートで行ってください。
また再ビルド時にのみ使用してください。
具体的には下記のことが行われます。<br>
1, cargo build --target x86_64-unknown-uefi を実行します。<br>
2, cp ./target/x86_64-unknown-uefi/debug/プロジェクト名.efi  ./esp/efi/boot/bootx64.efi を実行します。<br>
3, 下記を実行します。
```
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
```
