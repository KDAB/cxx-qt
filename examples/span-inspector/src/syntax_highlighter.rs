// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::inspector::qobject::{make_q_brush, make_q_text_char_format, QColor, QString};
use crate::inspector::TokenFlag;
use fancy_regex::Regex;
use std::pin::Pin;

/*
 * Stores highlight changes collected during parsing.
 * This allows applying all changes at once,
 * preventing them from being overwritten.
 */
#[derive(Default, Clone)]
struct PendingHighlights {
    foreground: Vec<Option<QColor>>,
    background: Vec<Option<QColor>>,
}

impl PendingHighlights {
    fn new(len: i32) -> Self {
        Self {
            foreground: vec![None; len as usize],
            background: vec![None; len as usize],
        }
    }

    fn set_foreground(&mut self, start: usize, len: usize, color: QColor) {
        for i in start..start + len {
            self.foreground[i] = Some(color.clone());
        }
    }

    fn set_background(&mut self, start: usize, len: usize, color: QColor) {
        for i in start..start + len {
            self.background[i] = Some(color.clone());
        }
    }
}

#[derive(Clone)]
struct HighlightingRule {
    regex: Regex,
    color: QColor,
}

impl HighlightingRule {
    fn new(regex: &str, r: i32, g: i32, b: i32) -> HighlightingRule {
        HighlightingRule {
            regex: Regex::new(regex).expect("Invalid regex pattern"),
            color: QColor::from_rgb(r, g, b),
        }
    }
}

pub struct SyntaxHighlighterRust {
    pub is_output: bool,
    pub char_flags: Option<Vec<TokenFlag>>,
    highlighting_rules: Vec<HighlightingRule>,
}

impl Default for SyntaxHighlighterRust {
    fn default() -> Self {
        Self {
            is_output: false,
            char_flags: None,
            highlighting_rules: vec![
                HighlightingRule::new(r"\w*::|None|Some|\d", 249, 152, 83),
                HighlightingRule::new(
                    r"(?<!\w)(use|struct|pub|impl|fn|Self|if|let|else|ref|mut|while|for|in|extern|type|unsafe|crate|match|loop|break|str|mod|usize|isize|char|bool|(u|i|f)\d{1, 2})(?!\w)",
                    255,
                    123,
                    144,
                ),
                HighlightingRule::new(r"\->|=>|\+=|-=|!|&|=|<|>|\*", 64, 126, 207),
                HighlightingRule::new(r"fn\s+(\w+)", 111, 192, 244),
                HighlightingRule::new(r"fn", 255, 123, 144),
                HighlightingRule::new(r"//.*", 103, 132, 181),
            ],
        }
    }
}

#[derive(PartialEq, Debug)]
#[repr(i32)]
enum State {
    Default = 0,
    Comment = 1,
    Literal = 2,
    Macro = 3,
}

impl From<i32> for State {
    fn from(value: i32) -> Self {
        match value {
            1 => State::Comment,
            2 => State::Literal,
            3 => State::Macro,
            _ => State::Default,
        }
    }
}

impl crate::inspector::qobject::SyntaxHighlighter {
    pub fn highlight_block(mut self: Pin<&mut Self>, text: &QString) {
        let text = text.to_string();
        let block_length = self.as_mut().current_block().length();
        let mut pending_highlights = PendingHighlights::new(block_length);

        if self.is_output {
            match self.as_mut().char_flags.clone() {
                Some(char_flags) => {
                    self.as_mut()
                        .highlight_regex(&text, &mut pending_highlights);
                    self.as_mut()
                        .highlight_multi_line(&text, &mut pending_highlights);
                    self.as_mut()
                        .highlight_char_flags(char_flags, &mut pending_highlights);
                }
                None => {
                    self.as_mut().highlight_error(&mut pending_highlights);
                }
            };
        } else {
            self.as_mut()
                .highlight_regex(&text, &mut pending_highlights);
            self.as_mut()
                .highlight_multi_line(&text, &mut pending_highlights);
        }

        self.as_mut().apply_highlights(&mut pending_highlights);
    }

    fn apply_highlights(mut self: Pin<&mut Self>, pending_highlights: &mut PendingHighlights) {
        let block_length = self.as_mut().current_block().length() as usize;
        for i in 0..block_length {
            let mut fmt = make_q_text_char_format();

            if let Some(color) = &pending_highlights.foreground[i] {
                fmt.pin_mut().set_foreground(&make_q_brush(color));
            }

            if let Some(color) = &pending_highlights.background[i] {
                fmt.pin_mut().set_background(&make_q_brush(color));
            }

            self.as_mut().set_format(i as i32, 1, &fmt);
        }
    }

    fn highlight_error(mut self: Pin<&mut Self>, pending_highlights: &mut PendingHighlights) {
        let block_length = self.as_mut().current_block().length() as usize;
        pending_highlights.set_foreground(0, block_length, QColor::from_rgb(255, 0, 0));
    }

    fn highlight_char_flags(
        mut self: Pin<&mut Self>,
        flags: Vec<TokenFlag>,
        pending_highlights: &mut PendingHighlights,
    ) {
        let block_length = self.as_mut().current_block().length();
        let block_position = self.as_mut().current_block().position();

        for i in 0..block_length {
            let color = match flags[(block_position + i) as usize] {
                TokenFlag::Original => QColor::from_rgba(0, 100, 155, 170),
                TokenFlag::Generated => QColor::from_rgba(0, 255, 0, 15),
                TokenFlag::Highlighted => QColor::from_rgba(255, 0, 0, 155),
            };
            pending_highlights.set_background(i as usize, 1, color);
        }
    }

    fn highlight_regex(
        self: Pin<&mut Self>,
        text: &str,
        pending_highlights: &mut PendingHighlights,
    ) {
        let matches: Vec<_> = self
            .highlighting_rules
            .iter()
            .flat_map(|rule| {
                rule.regex
                    .captures_iter(text)
                    .filter_map(Result::ok)
                    .map(|capture| (rule.color.clone(), capture.get(0).unwrap()))
            })
            .collect();

        for (color, mat) in matches.iter() {
            pending_highlights.set_foreground(mat.start(), mat.end() - mat.start(), color.clone());
        }
    }

    fn highlight_multi_line(
        mut self: Pin<&mut Self>,
        text: &str,
        pending_highlights: &mut PendingHighlights,
    ) {
        //                                        /*     | */ |     "     | #[ | ]
        let mut matches: Vec<_> = Regex::new("(?<!\\\\)/\\*|\\*/|(?<!\\\\)\"|#\\[|\\]")
            .unwrap()
            .find_iter(text)
            .filter_map(Result::ok)
            .collect();
        matches.sort_by_key(|m| m.start());

        let color_comment = QColor::from_rgb(103, 132, 181);
        let color_literal = QColor::from_rgb(111, 192, 244);
        let color_macro = QColor::from_rgb(176, 179, 11);

        let mut current_state = self.as_mut().previous_block_state().into();

        let mut highlight_start = 0;

        for mat in matches {
            let (color, next_state) = match (&current_state, mat.as_str()) {
                (State::Default, "/*") => {
                    highlight_start = mat.start();
                    (None, State::Comment)
                }

                (State::Default, "\"") => {
                    highlight_start = mat.start();
                    (None, State::Literal)
                }

                (State::Default, "#[") => {
                    highlight_start = mat.start();
                    (None, State::Macro)
                }

                (State::Comment, "*/") => (Some(color_comment.clone()), State::Default),
                (State::Literal, "\"") => (Some(color_literal.clone()), State::Default),
                (State::Macro, "]") => (Some(color_macro.clone()), State::Default),
                _ => (None, current_state),
            };

            if let Some(color) = color {
                let capture_length = mat.end() - highlight_start;
                pending_highlights.set_foreground(highlight_start, capture_length, color);
            }
            current_state = next_state;
        }

        let (color, next_state) = match current_state {
            State::Comment => (Some(color_comment), 1),
            State::Literal => (Some(color_literal), 2),
            State::Macro => (Some(color_macro), 3),
            State::Default => (None, 0),
        };

        if let Some(color) = color {
            pending_highlights.set_foreground(highlight_start, text.len() - highlight_start, color);
        }

        self.as_mut().set_current_block_state(current_state as i32);
    }
}
