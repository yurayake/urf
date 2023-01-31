# urf
uefi-rsを用いたプロジェクトを簡単にビルド、実行するツール

### サポートOS
GNU/Linux

### 使い方
```
urf [OPTIONS]

Options:
      --new <project_name>  
      --frun                
      --rrun                
  -h, --help
  -V, --version
```
#### -\-new
 新しいcargoプロジェクトを作成し、uefi-rsをセットアップします。
#### -\-frun
 プロジェクトをビルドし、qemu上で実行します。
なおこのコマンドはプロジェクトルートで行ってください。
また初めてのビルド&実行時以外に使用しないでください
#### -\-rrun
プロジェクトを再ビルドし、qemu上で実行します。
なおこのコマンドはプロジェクトルートで行ってください。
また再ビルド時にのみ使用してください。

### 詳細
コマンドの詳細についてはdocs/docs.mdをご覧ください。

### ライセンス
MIT