# rmz-text-compiler

Text compiler for Rockman Zero.

今のところ、ゼロ3のみ対応

```txt
<ルージュ>
  ジュンビは、よろしいでしょうか？
    はい
    いいえ
```

```sh
  0xF3, 0x62, 0x8D, 0xBA, 0xC6, 0xA8, 0x53, 0xD5, 0x6C, 0x71, 0x3B, 0x28, 0x4B, 0x3B, 0x6B, 0x2A, 0x2F, 0xDD, 0xFC, 0x00, 0x00, 0x53, 0x28, 0xFC, 0x00, 0x00, 0x28, 0x28, 0x2C, 0xFF
```

## Usage

```sh
# compile zero3 text
> rmz-text-compiler "ZERO3_TEXT" > output.bin

# compile zero3 text file
> rmz-text-compiler -f input.txt > output.bin

# compile zero3 chat file
> rmz-text-compiler -c -f input.zc > output.bin
```

## Syntax

```
赤文字
  <RED>XXXX</RED>
   -> 0xF2 ... 0xF1

NEXT
  ▼\n
    -> 0xFD

EOF or #
  -> 0xFF

prefix
  r: マグショットを右に
  b: チャットウィンドウを下に
    終端記号が来るまで有効
    -> 0xFB (BOTTOM)
```

## 拡張子

```
.txt -> ゼロ3テキストファイル(複数のテキストをまとめる場合もある 区切りは#)
.zc -> ゼロ3チャットファイル(後述)
```

## チャットファイル(.zc)

チャットファイルは

```txt
<r:ジョーヌ>
  ゼロさんが今までカイシュウされた
  データの入力....▼
  今、終わりました▼
<ルージュ>
  カイセキにうつります
```

のように、

```txt
<キャラクター>
  TEXT...
```

の形式のテキストファイルです。

チャットファイルはインデントの処理が、普通のテキストファイルと違っています。本来のテキストファイルだと、上のテキストは

```
<r:ジョーヌ>ゼロさんが今までカイシュウされた
データの入力....▼
今、終わりました▼
<ルージュ>
カイセキにうつります
```

のようになりますが、これだと読みづらいためチャットファイルを設けました。

## 注意

- ファイルの終端に余計な空行がないようにすること
- NEXT(`▼`)の次にはインデントが無い空行を置く事
- 複数テキストを1ファイルにまとめたいときは、`#`行で区切る(インデントなし)
