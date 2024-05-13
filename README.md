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

- `=` の左を key, 右を value とした HashMap を作成する
- `#` 始まりの行はコメント
- `;` 始まりの行はコメント
- パースできない行がある場合はプログラムを終了する
- パースできない行の行頭に `-` がある場合は、その行を無視してプログラムを続行する

https://man7.org/linux/man-pages/man5/sysctl.conf.5.html

## スキーマファイル

下記形式で、sysctl.conf の value の形式を指定する

```
endpoint -> string
debug -> bool
log.file -> string
```

- `->` の左に key, 右に形式を指定する
- 指定できる形式は以下
    - string: 文字列
    - bool: 真偽値（`true`, `false` の二値）
    - int: 整数値（i32 の範囲）
    - float: 浮動小数点値（f64 の範囲）

## run

第一引数: sysctl.conf ファイルのパス

第二引数: schema ファイルのパス

スキーマなし

```bash
cargo run resources/test/sysctl/sysctl.conf
```

スキーマあり

```bash
# スキーマに合致する場合
cargo run resources/test/sysctl/schema/sysctl_valid.conf resources/test/sysctl/schema/schema.txt

# スキーマに合致しない場合
cargo run resources/test/sysctl/schema/sysctl_invalid.conf resources/test/sysctl/schema/schema.txt
```
