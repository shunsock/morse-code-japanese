use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MorseCodeError {
    UnknownCharacter(char),
    UnknownMorseCode(String),
}

impl std::fmt::Display for MorseCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MorseCodeError::UnknownCharacter(c) => write!(f, "Unknown character: {}", c),
            MorseCodeError::UnknownMorseCode(code) => write!(f, "Unknown Morse code: {}", code),
        }
    }
}

impl std::error::Error for MorseCodeError {}

pub struct MorseCode {
    to_morse: HashMap<char, &'static str>,
    from_morse: HashMap<&'static str, char>,
    dot: String,
    dash: String,
}

impl MorseCode {
    pub fn new(dot: Option<&str>, dash: Option<&str>) -> Self {
        let mut to_morse = HashMap::new();
        let mut from_morse = HashMap::new();

        let mappings = vec![
            ('イ', ".-"),
            ('ロ', ".-.-"),
            ('ハ', "-..."),
            ('ニ', "-.-."),
            ('ホ', "-.."),
            ('ヘ', "."),
            ('ト', "..-.."),
            ('チ', "..-."),
            ('リ', "--."),
            ('ヌ', "...."),
            ('ル', "-.--."),
            ('ヲ', ".---"),
            ('ワ', "-.-"),
            ('カ', ".-.."),
            ('ヨ', "--"),
            ('タ', "-."),
            ('レ', "---"),
            ('ソ', "---."),
            ('ツ', ".--."),
            ('ネ', "--.-"),
            ('ナ', ".-."),
            ('ラ', "..."),
            ('ム', "-"),
            ('ウ', "..-"),
            ('ヰ', ".-..-"),
            ('ノ', "..--"),
            ('オ', ".-..."),
            ('ク', "...-"),
            ('ヤ', ".--"),
            ('マ', "-..-"),
            ('ケ', "-.--"),
            ('フ', "--.."),
            ('コ', "----"),
            ('エ', "-.---"),
            ('テ', ".-.--"),
            ('ア', "--.--"),
            ('サ', "-.-.-"),
            ('キ', "-.-.."),
            ('ユ', "-..--"),
            ('メ', "-...-"),
            ('ミ', "..-.-"),
            ('シ', "--.-."),
            ('ヱ', ".--.."),
            ('ヒ', "--..-"),
            ('モ', "-..-."),
            ('セ', ".---."),
            ('ス', "---.-"),
            ('ン', ".-.-."),
            ('゛', ".."),
            ('゜', "..--."),
        ];

        for (char, morse) in mappings {
            to_morse.insert(char, morse);
            from_morse.insert(morse, char);
        }

        MorseCode {
            to_morse,
            from_morse,
            dot: dot.unwrap_or(".").to_string(),
            dash: dash.unwrap_or("-").to_string(),
        }
    }

    fn replace(&self, code: &str) -> String {
        code.chars()
            .map(|c| match c {
                '.' => self.dot.clone(),
                '-' => self.dash.clone(),
                _ => c.to_string(),
            })
            .collect()
    }

    pub fn encode(&self, text: &str) -> Result<String, MorseCodeError> {
        text.chars()
            .map(|c| {
                self.to_morse
                    .get(&c)
                    .map(|s| self.replace(s))
                    .ok_or(MorseCodeError::UnknownCharacter(c))
            })
            .collect::<Result<Vec<String>, _>>()
            .map(|v| v.join(" "))
    }

    pub fn decode(&self, morse: &str) -> Result<String, MorseCodeError> {
        morse
            .split_whitespace()
            .map(|code| {
                let replaced: String = code
                    .chars()
                    .collect::<String>()
                    .replace(&self.dot, ".")
                    .replace(&self.dash, "-");

                self.from_morse
                    .get(replaced.as_str())
                    .copied()
                    .ok_or(MorseCodeError::UnknownMorseCode(code.to_string()))
            })
            .collect::<Result<Vec<char>, _>>()
            .map(|v| v.into_iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let morse = MorseCode::new(Some("."), Some("-"));
        assert_eq!(morse.encode("イロハ").unwrap(), ".- .-.- -...");
        assert_eq!(morse.encode("ヘ").unwrap(), ".");
        assert!(matches!(
            morse.encode("あ"),
            Err(MorseCodeError::UnknownCharacter('あ'))
        ));
    }

    #[test]
    fn test_decode() {
        let morse = MorseCode::new(Some("."), Some("-"));
        assert_eq!(morse.decode(".- .-.- -...").unwrap(), "イロハ");
        assert_eq!(morse.decode(".").unwrap(), "ヘ");
        assert!(matches!(
            morse.decode("...---"),
            Err(MorseCodeError::UnknownMorseCode(ref code)) if code == "...---"
        ));
    }

    #[test]
    fn test_custom_symbols() {
        let morse = MorseCode::new(Some("*"), Some("~"));
        assert_eq!(morse.encode("イロハ").unwrap(), "*~ *~*~ ~***");
        assert_eq!(morse.decode("*~ *~*~ ~***").unwrap(), "イロハ");
    }
}
