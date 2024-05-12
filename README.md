# read-sysctl

下記のようなファイルを読み込み、HashMap に格納する

```
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log

# comment
; comment
```

## 仕様

https://man7.org/linux/man-pages/man5/sysctl.conf.5.html

## run

第一引数: sysctl.conf ファイルのパス
第二引数: schema ファイルのパス

スキーマなし

```bash
cargo run resources/sysctl.conf
```

スキーマあり

```bash
cargo run resources/sysctl.conf resources/schema.txt
```
