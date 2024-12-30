# morse-code-japanese

Simple Morse code converter for Japanese text. (和文モールス信号変換器) You can see ths table of Morse code for Japanese characters [here](https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7)

## Usage

By default, the library uses dot and dash for Morse code.

```rust
use morse_code_japanese::MorseCode;

fn main() {
    let morse = MorseCode::new(None, None);

    let raw: &str = "コンニチハセカイ";
    println!("raw: {:?}", raw.clone());

    let encoded: String = morse.encode(&raw).unwrap();
    println!("encoded: {:?}", encoded);

    let decoded: String = morse.decode(&encoded).unwrap();
    println!("decoded: {:?}", decoded);
}
```

This will output:

```shell
raw: "コンニチハセカイ"
encoded: "---- .-.-. -.-. ..-. -... .---. .-.. .-"
decoded: "コンニチハセカイ"
```

You can also use other characters for Morse code.

```rust
use morse_code_japanese::MorseCode;

fn main() {
    let morse = MorseCode::new(Some("🇯🇵"), Some("🗻"));

    let raw: &str = "コンニチハセカイ";
    println!("raw: {:?}", raw.clone());

    let encoded: String = morse.encode(&raw).unwrap();
    println!("encoded: {:?}", encoded);

    let decoded: String = morse.decode(&encoded).unwrap();
    println!("decoded: {:?}", decoded);
}
```

This will output:

```shell
raw: "コンニチハセカイ"
encoded: "🗻🗻🗻🗻 🇯🇵🗻🇯🇵🗻🇯🇵 🗻🇯🇵🗻🇯🇵 🇯🇵🇯🇵🗻🇯🇵 🗻🇯🇵🇯🇵🇯🇵 🇯🇵🗻🗻🗻🇯🇵 🇯🇵🗻🇯🇵🇯🇵 🇯🇵🗻"
decoded: "コンニチハセカイ"
```

Yes! it's more like [COOL JAPAN](https://en.wikipedia.org/wiki/Cool_Japan)!

## License

[MIT](./LICENSE)

