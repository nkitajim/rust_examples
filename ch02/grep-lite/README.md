# create
```
cargo add clap
cargo add regex
```
# &str method
```
- 1. 長さ・判定系
len()	バイト数を返す
is_empty()	空文字かどうか
chars().count()	文字数（Unicodeスカラ値）
bytes().count()	バイト数（len() と同じ）
contains(&str)	部分文字列が含まれるか
starts_with(&str)	指定文字列で始まるか
ends_with(&str)	指定文字列で終わるか

- 2. 分割・反復系
lines()	改行で分割して行イテレータを返す
split(pattern)	区切り文字・パターンで分割
split_whitespace()	空白文字で分割
splitn(n, pattern)	n 回だけ分割
rsplit(pattern)	右から分割
matches(pattern)	部分一致のイテレータ
match_indices(pattern)	部分一致と位置のタプルイテレータ
char_indices()	文字とバイト位置のイテレータ
chars()	Unicode スカラ値ごとのイテレータ
bytes()	バイトごとのイテレータ

- 3. 検索系
find(&str)	最初の一致位置（バイトインデックス）
rfind(&str)	最後の一致位置
starts_with(&str)	指定文字列で始まるか
ends_with(&str)	指定文字列で終わるか
contains(&str)	部分文字列が含まれるか

- 4. 置換・変換系
replace(from, to)	全置換
replacen(from, to, n)	n 回だけ置換
trim()	前後空白削除
trim_start()	先頭空白削除
trim_end()	末尾空白削除
to_uppercase()	大文字化（Unicode対応）
to_lowercase()	小文字化（Unicode対応）
to_string()	String に変換
to_owned()	String に変換（to_string() と同等）

- 5. サブ文字列・スライス
get(range)	バイト範囲で文字列を取得（安全）
get(index..index)	文字列スライスを取得
split_at(index)	指定バイト位置で前後に分割

- 6. 比較・検索
eq(&str)	等しいか
ne(&str)	等しくないか
starts_with(&str)	指定文字列で始まるか
ends_with(&str)	指定文字列で終わるか
contains(&str)	部分文字列が含まれるか
find(&str)	最初に現れる位置（バイトインデックス）

- 7. 注意点

&str は 不変なので編集できません（挿入や削除はできない）

編集する場合は String に変換する必要があります
```
