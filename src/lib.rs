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
    dot: char,
    dash: char,
}

impl MorseCode {
    pub fn new(dot: Option<char>, dash: Option<char>) -> Self {
        let mut to_morse = HashMap::new();
        let mut from_morse = HashMap::new();

        let mappings = vec![
            ('イ', ".-"), ('ロ', ".-.-"), ('ハ', "-..."), ('ニ', "-.-."), ('ホ', "-.."), ('ヘ', "."),
            ('ト', "..-.."), ('チ', "..-."), ('リ', "--."), ('ヌ', "...."), ('ル', "-.--."),
            ('ヲ', ".---"), ('ワ', "-.-"), ('カ', ".-.."), ('ヨ', "--"), ('タ', "-."), ('レ', "---"),
            ('ソ', "---."), ('ツ', ".--."), ('ネ', "--.-"), ('ナ', ".-."), ('ラ', "..."),
            ('ム', "-"), ('ウ', "..-"), ('ヰ', ".-..-"), ('ノ', "..--"), ('オ', ".-..."),
            ('ク', "...-"), ('ヤ', ".--"), ('マ', "-..-"), ('ケ', "-.--"), ('フ', "--.."),
            ('コ', "----"), ('エ', "-.---"), ('テ', ".-.--"), ('ア', "--.--"), ('サ', "-.-.-"),
            ('キ', "-.-.."), ('ユ', "-..--"), ('メ', "-...-"), ('ミ', "..-.-"), ('シ', "--.-."),
            ('ヱ', ".--.."), ('ヒ', "--..-"), ('モ', "-..-."), ('セ', ".---."), ('ス', "---.-"),
            ('ン', ".-.-."), ('゛', ".."), ('゜', "..--."),
        ];

        for (char, morse) in mappings {
            to_morse.insert(char, morse);
            from_morse.insert(morse, char);
        }

        MorseCode {
            to_morse,
            from_morse,
            dot: dot.unwrap_or('.'),
            dash: dash.unwrap_or('-'),
        }
    }

    fn replace(&self, code: &str) -> String {
        code.chars()
            .map(|c| match c {
                '.' => self.dot,
                '-' => self.dash,
                _ => c,
            })
            .collect()
    }

    pub fn encode(&self, text: &[char]) -> Result<Vec<String>, MorseCodeError> {
        text.iter()
            .map(|&c| {
                self.to_morse
                    .get(&c)
                    .map(|s| self.replace(s))
                    .ok_or(MorseCodeError::UnknownCharacter(c))
            })
            .collect()
    }

    pub fn decode(&self, morse: &[&str]) -> Result<Vec<char>, MorseCodeError> {
        morse.iter()
            .map(|&code| {
                let replaced: String = code
                    .chars()
                    .map(|c| match c {
                        c if c == self.dot => '.',
                        c if c == self.dash => '-',
                        _ => c,
                    })
                    .collect();
                self.from_morse
                    .get(replaced.as_str())
                    .copied()
                    .ok_or(MorseCodeError::UnknownMorseCode(code.to_string()))
            })
            .collect()
    }

    pub fn encode_from_string(&self, text: &str) -> Result<String, MorseCodeError> {
        self.encode(&text.chars().collect::<Vec<_>>())
            .map(|v| v.join(" "))
    }

    pub fn decode_to_string(&self, text: &str) -> Result<String, MorseCodeError> {
        self.decode(&text.split_whitespace().collect::<Vec<_>>())
            .map(|v| v.into_iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_morse() {
        let morse = MorseCode::new(None, None);
        let result: Result<Vec<char>, MorseCodeError> = morse.decode(&["...."]);
        assert_eq!(result, Ok(vec!['ヌ']));
    }

    #[test]
    fn test_single_characters() {
        let morse = MorseCode::new(None, None);
        let input: Vec<char> = vec!['ア', 'イ', 'ウ'];
        let encoded: Result<Vec<String>, MorseCodeError> = morse.encode(&input);
        assert_eq!(encoded.unwrap(), vec!["--.--", ".-", "..-"]);

        let decoded: Result<Vec<char>, MorseCodeError> = morse.decode(&["--.--", ".-", "..-"]);
        assert_eq!(decoded.unwrap(), input);
    }

    #[test]
    fn test_custom_symbols() {
        let morse = MorseCode::new(Some('*'), Some('/'));
        let input: Vec<char> = vec!['ア', 'イ', 'ウ'];
        let encoded: Result<Vec<String>, MorseCodeError> = morse.encode(&input);
        assert_eq!(encoded.unwrap(), vec!["//*//", "*/", "**/"]);

        let decoded: Result<Vec<char>, MorseCodeError> = morse.decode(&["//*//", "*/", "**/"]);
        assert_eq!(decoded.unwrap(), input);
    }

    #[test]
    fn test_encode_from_string() {
        let morse = MorseCode::new(None, None);
        let result: Result<String, MorseCodeError> = morse.encode_from_string("アイウ");
        assert_eq!(result.unwrap(), "--.-- .- ..-");
    }

    #[test]
    fn test_decode_to_string() {
        let morse = MorseCode::new(None, None);
        let result: Result<String, MorseCodeError> = morse.decode_to_string("--.-- .- ..-");
        assert_eq!(result.unwrap(), "アイウ");
    }
}
