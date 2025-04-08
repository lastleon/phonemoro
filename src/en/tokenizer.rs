use std::ops::Range;

use anyhow::{anyhow, Result};
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // WORD TOKEN
    /// Represents an entire word within the input sequence. An apostrophe (') is considered
    /// to be part of the word, if it is followed by word character. There is no limit to how many apostrophes
    /// can be in a word, they just need to follow the previous rule. This exists so that words like "haven't"
    /// are kept together. Note that this also includes characters like "ä", which might not be usable by the phonemizer FST.
    ///
    ///
    /// _Simplified regex_:
    ///
    /// `word_char_no_digits+ ((apostrophe)(word_char_no_digits))*`
    #[regex(r"[[\w]&&[^\d]]+(?:'[[\w]&&[^\d]]+)*")]
    Word,

    // TODO: Add other abbreviations
    #[regex(r"(?:(i:dr)|(i:ms)|(i:mr))\.")]
    AbbreviatedWord,

    // NUMBER TOKENS
    #[regex(r"\d+")]
    DigitSequence,

    // TODO: Date.
    // Multiple versions need to be considered: 01.02.2024, 1.2.2024, 01.02.24, ...
    // Date
    /// Represents a decimal number with digits before and after the decimal point.
    #[regex(r"\d+\.\d+")]
    DecimalNumber,

    // SPECIAL TOKENS
    /// All characters considered to be sentence delimiters. Note that dot can be ambiguous,
    /// such as in links for example (www.google.com). This situation needs to be fixed,
    /// but is currently not simple, since Logos does not implement lookahead.
    #[regex(r"(?:[\.!?])+")]
    SentenceDelimiter,

    // TODO:
    // Dot
    #[regex(r##"[\$\%\&\=\*\+\;<>|\^"'~:\/\\#,()\[\]\{\}-]"##)] // correct? should be ".,!?()[]{}-
    Special,

    // Currencies? So much more probably..

    // EVERYTHING ELSE
    #[regex(r"\s+")]
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub struct TokenContext<'a> {
    pub token: Token,
    pub slice: &'a str,
    pub span: Range<usize>,
}

impl<'a> From<&TokenContext<'a>> for (Token, &'a str) {
    /// Convert TokenContext to tuple of token and &str. Mainly used for testing.
    fn from(value: &TokenContext<'a>) -> Self {
        (value.token.clone(), value.slice)
    }
}

pub struct EnTokenizer;
impl EnTokenizer {
    pub fn tokenize<'a>(text: &'a str) -> Result<Vec<TokenContext<'a>>> {
        let mut lex = Token::lexer(text);

        let mut tokens: Vec<TokenContext<'a>> = Vec::new();
        while let Some(lex_result) = lex.next() {
            tokens.push(TokenContext {
                token: lex_result.map_err(|_| {
                    anyhow!(
                        "Lexing error. Data:\nCurrent lexer slice: {}\nRemaining text:{}",
                        lex.slice(),
                        lex.remainder()
                    )
                })?,
                slice: lex.slice(),
                span: lex.span(),
            });
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Convenience macro to convert Vec<TokenContext> to Vec<(Token, &str)>
    /// for easier comparison when testing.
    macro_rules! to_tuple {
        ($a:expr) => {
            $a.into_iter()
              .map(|v| (&v).into())
              .collect::<Vec<(Token, &str)>>()
        };
    }

    #[test]
    fn basic_tokenization_test() {
        let text_input = "Hello there";
        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::Word, "Hello"),
            (Token::Whitespace, " "),
            (Token::Word, "there"),
        ];
        assert_eq!(output, expected)
    }

    #[test]
    fn apostrophe_within_word_test() {
        let text_input = "first'second";
        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![(Token::Word, "first'second")];
        assert_eq!(output, expected)
    }

    #[test]
    fn two_apostrophe_within_word() {
        let text_input = "word''word";

        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::Word, "word"),
            (Token::Special, "'"),
            (Token::Special, "'"),
            (Token::Word, "word"),
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn apostrophes_outside_word() {
        let text_input = "'word'";

        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::Special, "'"),
            (Token::Word, "word"),
            (Token::Special, "'"),
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn num_test() {
        let text_input = "1234w1234";

        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::DigitSequence, "1234"),
            (Token::Word, "w"),
            (Token::DigitSequence, "1234"),
        ];

        assert_eq!(output, expected)
    }

    #[test]
    fn decimal_number_basic_test() {
        let text_input = "123.0";

        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![(Token::DecimalNumber, "123.0")];

        assert_eq!(output, expected)
    }

    #[test]
    fn decimal_number_no_digits_after_decimal_test() {
        let text_input = "123.";

        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::DigitSequence, "123"),
            (Token::SentenceDelimiter, "."),
        ];

        assert_eq!(output, expected)
    }

    #[test]
    fn sentence_delimiter_basic_test() {
        let text_input = "word.";
        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![(Token::Word, "word"), (Token::SentenceDelimiter, ".")];
        assert_eq!(output, expected);
    }

    #[test]
    fn sentence_delimiter_longer_test() {
        let text_input = "word. word";
        let output = to_tuple!(EnTokenizer::tokenize(text_input).unwrap());
        let expected = vec![
            (Token::Word, "word"),
            (Token::SentenceDelimiter, "."),
            (Token::Whitespace, " "),
            (Token::Word, "word"),
        ];
        assert_eq!(output, expected);
    }
}
