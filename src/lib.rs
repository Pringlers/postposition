use std::fmt::Display;

use aho_corasick::AhoCorasick;

const NUM_WITH_FINAL_CONSONANT: [char; 6] = ['0', '1', '3', '6', '7', '8'];
#[rustfmt::skip]
const KO_INCOMPLETE_PATTERN: [&str; 51] = [
    "ㄳ", "ㄵ", "ㄶ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅄ",
    "ㄱ", "ㄴ", "ㄷ", "ㄹ", "ㅁ", "ㅂ", "ㅅ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ",
    "ㄲ", "ㄸ", "ㅃ", "ㅆ", "ㅉ",
    "ㅏ", "ㅓ", "ㅗ", "ㅜ", "ㅡ", "ㅣ", "ㅐ", "ㅔ", "ㅚ", "ㅟ", "ㅑ", "ㅕ", "ㅛ", "ㅠ",
    "ㅒ", "ㅖ", "ㅘ", "ㅙ", "ㅝ", "ㅞ", "ㅢ"
];
#[rustfmt::skip]
const KO_COMPLETE_PATTERN: [&str; 51] = [
    "ㄱㅅ", "ㄴㅈ", "ㄴㅎ", "ㄹㄱ", "ㄹㅁ", "ㄹㅂ", "ㄹㅅ", "ㄹㅌ", "ㄹㅍ", "ㄹㅎ", "ㅂㅅ",
    "기역", "니은", "디귿", "리을", "미음", "비읍", "시옷", "이응", "지읒", "치읓", "키읔", "티읕", "피읖", "히읗",
    "쌍기역", "쌍디귿", "쌍비읍", "쌍시옷", "쌍지읒",
    "아", "어", "오", "우", "으", "이", "애", "에", "외", "위", "야", "여", "요", "유",
    "얘", "예", "와", "왜", "워", "웨", "의"
];

macro_rules! define_regex {
    ($name:ident, $regex:expr) => {
        static $name: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($regex).unwrap());
    };
}

define_regex!(UNKNOWN_CHARS, r"[^ㄱ-ㅎㅏ-ㅣ가-힣a-zA-Z\d]");
define_regex!(HANGEULS, r"[가-힣]$");
define_regex!(KO_WITH_NUM, r"[가-힣]\d*[013678]$");
define_regex!(EN_WITH_NUM, r"[a-zA-Z]\d*[1789]$");
define_regex!(
    EN_FINAL_CONSONANT,
    r"([clmnp]|[blnt](e)|[co](k)|[aeiou](t)|mb|ng|lert)$"
);

/// Returns `consonant` if `input` ends with consonant, `vowel` if not.
///
/// When `input` is full of unsupported characters or empty,
/// this method will return an empty `&str`.
///
/// English support is not perfect, but provided on a best-efforts basis.
///
/// # Examples
///
/// ```
/// use postposition::josa;
///
/// assert_eq!(josa("홍길동", "이", "가"), "이");
/// assert_eq!(josa("Yuna", "아", "야"), "야");
/// assert_eq!(josa("こんにちは", "을", "를"), "");
/// ```
pub fn josa<'a>(input: &str, consonant: &'a str, vowel: &'a str) -> &'a str {
    let input = clean_str(input);
    if input.is_empty() {
        return "";
    }

    if ends_with_consonant(&input) {
        consonant
    } else {
        vowel
    }
}

/// Utility trait to find or attach postpositions.
///
/// # Examples
/// ```
/// use std::io;
/// use postposition::Postposition;
///
/// # fn main() -> io::Result<()> {
/// let mut buffer = String::new();
/// let stdin = io::stdin();
/// stdin.read_line(&mut buffer)?;
/// println!(
///     "당신은 {input} 입력했습니다.",
///     input = buffer.trim().attached("을", "를")
/// );
/// #     Ok(())
/// # }
/// ```
pub trait Postposition: Display {
    /// This is the equivalent of [`josa`].
    ///
    /// [`josa`]: crate::josa
    ///
    /// # Examples
    ///
    /// ```
    /// use postposition::Postposition;
    ///
    /// assert_eq!("딸기맛 바나나".josa("이", "가"), "가");
    /// assert_eq!(24.josa("은", "는"), "는");
    /// ```
    fn josa<'a>(&self, consonant: &'a str, vowel: &'a str) -> &'a str;

    /// Attaches an appropriate postposition at the end.
    ///
    /// When `self` is full of unsupported characters or empty,
    /// this method will return `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use postposition::Postposition;
    ///
    /// assert_eq!("딸기맛 바나나".attached("이", "가"), "딸기맛 바나나가");
    /// assert_eq!(24.attached("은", "는"), "24는");
    /// ```
    fn attached(&self, consonant: &str, vowel: &str) -> String {
        let postposition = self.josa(consonant, vowel);
        if postposition.is_empty() {
            return self.to_string();
        }
        format!("{self}{postposition}")
    }
}

impl Postposition for str {
    fn josa<'a>(&self, consonant: &'a str, vowel: &'a str) -> &'a str {
        josa(self, consonant, vowel)
    }
}

macro_rules! postposition_impl {
    ($t:ty) => {
        impl Postposition for $t {
            fn josa<'a>(&self, consonant: &'a str, vowel: &'a str) -> &'a str {
                josa(&self.to_string(), consonant, vowel)
            }
        }
    };
}

postposition_impl!(char);

postposition_impl!(i8);
postposition_impl!(i16);
postposition_impl!(i32);
postposition_impl!(i64);
postposition_impl!(isize);

postposition_impl!(u8);
postposition_impl!(u16);
postposition_impl!(u32);
postposition_impl!(u64);
postposition_impl!(usize);

fn ends_with_consonant(input: &str) -> bool {
    let last_char = match input.chars().last() {
        Some(c) => c,
        None => unreachable!(), // should have returned empty &str
    };

    (HANGEULS.is_match(input) && (last_char as u32 - 0xac00) % 28 > 0)
        || KO_WITH_NUM.is_match(input) // checks numbers in Korean pronunciation
        || EN_WITH_NUM.is_match(input) // checks numbers in English pronunciation
        || (input.len() > 1 && EN_FINAL_CONSONANT.is_match(input)) // checks if English syllable ends with consonant
        || (input.len() == 1 && ['l', 'n', 'm', 'r'].contains(&last_char)) // checks if single alphabet ends with consonant
        || ends_with_consonant_digit(input, &last_char) // checks if last digit in the number ends with consonant
}

fn ends_with_consonant_digit(input: &str, last_char: &char) -> bool {
    let word = match input.split_whitespace().last() {
        Some(word) => word,
        None => return false,
    };
    word.chars().all(|c| c.is_ascii_digit()) && NUM_WITH_FINAL_CONSONANT.contains(last_char)
}

fn clean_str(input: &str) -> String {
    let input = input.trim();
    if input.is_empty() {
        return String::new();
    }

    let haystack = UNKNOWN_CHARS.replace_all(input, "");
    let ac = AhoCorasick::new(&KO_INCOMPLETE_PATTERN);
    ac.replace_all(&haystack, &KO_COMPLETE_PATTERN)
}
