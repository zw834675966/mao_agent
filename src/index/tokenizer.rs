use jieba_rs::{Jieba, TokenizeMode};
use std::sync::Arc;
use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

/// Domain-specific Chinese Jieba tokenizer for Tantivy.
#[derive(Clone)]
pub struct JiebaTokenizer {
    jieba: Arc<Jieba>,
}

impl Default for JiebaTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl JiebaTokenizer {
    /// Create a new JiebaTokenizer with built-in historical and political domain lexicon.
    pub fn new() -> Self {
        let mut jieba = Jieba::new();

        // Inject domain-specific historical and philosophical terms
        let domain_words = [
            "统一战线",
            "武装斗争",
            "党的建设",
            "农村包围城市",
            "武装割据",
            "三大纪律八项注意",
            "三大法宝",
            "持久战",
            "亡国论",
            "速胜论",
            "战略防御",
            "战略相持",
            "战略反攻",
            "游击战争",
            "人民战争",
            "矛盾论",
            "实践论",
            "主要矛盾",
            "矛盾的主要方面",
            "对立统一",
            "群众路线",
            "实事求是",
            "独立自主",
            "自力更生",
            "工农武装割据",
            "新民主主义",
            "星星之火",
            "可以燎原",
            "没有调查就没有发言权",
            "惩前毖后",
            "治病救人",
            "百花齐放",
            "百家争鸣",
            "延安整风",
        ];

        for word in domain_words {
            jieba.add_word(word, None, None);
        }

        Self {
            jieba: Arc::new(jieba),
        }
    }
}

impl Tokenizer for JiebaTokenizer {
    type TokenStream<'a> = JiebaTokenStream;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        let words = self.jieba.tokenize(text, TokenizeMode::Search, true);
        let mut tokens = Vec::with_capacity(words.len());

        // Precompute single-pass char-to-byte offsets for O(1) token boundary lookups
        let mut char_to_byte: Vec<usize> = text.char_indices().map(|(b, _)| b).collect();
        char_to_byte.push(text.len());

        let byte_len = text.len();
        let lookup_offset = |char_idx: usize| -> usize {
            if char_idx < char_to_byte.len() {
                char_to_byte[char_idx]
            } else {
                byte_len
            }
        };

        for (i, token) in words.into_iter().enumerate() {
            let trimmed = token.word.trim();
            // Skip empty/pure whitespace tokens
            if trimmed.is_empty() || trimmed == "\n" || trimmed == "\r" {
                continue;
            }

            tokens.push(Token {
                offset_from: lookup_offset(token.start),
                offset_to: lookup_offset(token.end),
                position: i,
                text: token.word.to_string(),
                position_length: 1,
            });
        }

        JiebaTokenStream { tokens, index: 0 }
    }
}

pub struct JiebaTokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream for JiebaTokenStream {
    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.tokens[self.index - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jieba_tokenizer_domain_words() {
        let mut tokenizer = JiebaTokenizer::new();
        let text = "农村包围城市与武装夺取政权是统一战线的重要组成部分";
        let mut stream = tokenizer.token_stream(text);

        let mut tokens = Vec::new();
        while stream.advance() {
            tokens.push(stream.token().text.clone());
        }

        assert!(tokens.iter().any(|t| t == "农村包围城市"));
        assert!(tokens.iter().any(|t| t == "统一战线"));
    }

    #[test]
    fn test_jieba_search_mode_indexes_compound_subwords() {
        let mut tokenizer = JiebaTokenizer::new();
        let text = "中国走农村包围城市道路";
        let mut stream = tokenizer.token_stream(text);

        let mut tokens = Vec::new();
        while stream.advance() {
            tokens.push(stream.token().text.clone());
        }

        assert!(
            tokens.iter().any(|t| t == "农村包围城市"),
            "compound domain term must remain a token: {tokens:?}"
        );
        assert!(
            tokens.iter().any(|t| t == "农村"),
            "search mode must also index the subword 农村 so BM25 can match it: {tokens:?}"
        );
    }
}
