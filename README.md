# read-sysctl

下記のようなファイルを読み込み、HashMap に格納する

```
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log

# comment
; comment
```

## run

```bash
cargo run resources/sysctl.conf
```
