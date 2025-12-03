// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::inspector::qobject::{make_q_brush, make_q_text_char_format, QColor, QString};
use crate::inspector::TokenFlag;
use fancy_regex::Regex;
use std::pin::Pin;

#[derive(Default, Clone)]
struct FinalFormats {
    foreground: Vec<Option<QColor>>,
    background: Vec<Option<QColor>>,
}

impl FinalFormats {
    fn new(len: i32) -> Self {
        Self {
            foreground: vec![None; len as usize],
            background: vec![None; len as usize],
        }
    }

    fn set_foreground(&mut self, start: usize, end: usize, color: QColor) {
        for i in start..start + end {
            self.foreground[i] = Some(color.clone());
        }
    }

    fn set_background(&mut self, start: usize, end: usize, color: QColor) {
        for i in start..start + end {
            self.background[i] = Some(color.clone());
        }
    }
}

#[derive(Clone)]
struct HighlightingRule {
    regex: Regex,
    color: QColor,
}

fn rule(regex: &str, r: i32, g: i32, b: i32) -> HighlightingRule {
    HighlightingRule {
        regex: Regex::new(regex).unwrap(),
        color: QColor::from_rgb(r, g, b),
    }
}

pub struct SyntaxHighlighterRust {
    highlighting_rules: Vec<HighlightingRule>,
    pub is_output: bool,
    pub char_flags: Option<Vec<TokenFlag>>,
}

impl Default for SyntaxHighlighterRust {
    fn default() -> Self {
        Self {
            highlighting_rules: vec![
                rule(r"\w*::|None|Some|\d", 249, 152, 83),
                rule(
                    r"(?<!\w)(use|struct|pub|impl|fn|Self|if|let|else|ref|mut|while|for|in|extern|type|unsafe|crate|match|loop|break|str|mod|usize|isize|char|bool|(u|i|f)\d{1, 2})(?!\w)",
                    255,
                    123,
                    144,
                ),
                rule(r"\->|=>|\+=|-=|!|&|=|<|>|\*", 64, 126, 207),
                rule(r"fn\s+(\w+)", 111, 192, 244),
                rule(r"fn", 255, 123, 144),
                rule(r"//.*", 103, 132, 181),
            ],
            is_output: false,
            char_flags: None,
        }
    }
}

impl crate::inspector::qobject::SyntaxHighlighter {
    pub fn highlight_block(mut self: Pin<&mut Self>, text: &QString) {
        let text = text.to_string();
        let block_length = self.as_mut().current_block().length();
        let mut final_fmt = FinalFormats::new(block_length);

        if self.is_output == true {
            match self.as_mut().char_flags.clone() {
                Some(char_flags) => {
                    self.as_mut().highlight_regex(&mut final_fmt, &text);
                    self.as_mut().highlight_multi_line(&mut final_fmt, &text);
                    self.as_mut()
                        .highlight_char_flags(&mut final_fmt, char_flags);
                }
                None => {
                    self.as_mut().highlight_error(&mut final_fmt);
                }
            };
        } else {
            self.as_mut().highlight_regex(&mut final_fmt, &text);
            self.as_mut().highlight_multi_line(&mut final_fmt, &text);
        }

        self.as_mut().apply_formats(final_fmt);
    }

    fn apply_formats(mut self: Pin<&mut Self>, final_fmt: FinalFormats) {
        let block_length = self.as_mut().current_block().length() as usize;
        for i in 0..block_length {
            let mut fmt = make_q_text_char_format();

            if let Some(color) = &final_fmt.foreground[i] {
                fmt.pin_mut().set_foreground(&make_q_brush(color));
            }

            if let Some(color) = &final_fmt.background[i] {
                fmt.pin_mut().set_background(&make_q_brush(color));
            }

            self.as_mut().set_format(i as i32, 1, &*fmt);
        }
    }

    fn highlight_error(mut self: Pin<&mut Self>, final_fmt: &mut FinalFormats) {
        let block_length = self.as_mut().current_block().length();
        final_fmt.set_foreground(0, block_length as usize, QColor::from_rgb(255, 0, 0));
    }

    fn highlight_char_flags(
        mut self: Pin<&mut Self>,
        final_fmt: &mut FinalFormats,
        flags: Vec<TokenFlag>,
    ) {
        let block_length = self.as_mut().current_block().length();
        let block_position = self.as_mut().current_block().position();

        for i in 0..block_length {
            let color = match flags[(block_position + i) as usize] {
                TokenFlag::Original => QColor::from_rgb(0, 0, 255),
                TokenFlag::Generated => QColor::from_rgb(0, 255, 0),
                TokenFlag::Highlighted => QColor::from_rgb(255, 0, 0),
            };
            final_fmt.set_background(i as usize, 1, color);
        }
    }

    fn highlight_regex(self: Pin<&mut Self>, final_fmt: &mut FinalFormats, text: &str) {
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
            final_fmt.set_foreground(mat.start(), mat.end() - mat.start(), color.clone());
        }
    }

    fn highlight_multi_line(mut self: Pin<&mut Self>, final_fmt: &mut FinalFormats, text: &str) {
        //                                        /*     | */ |     "     | #[ | ]
        let mut matches: Vec<_> = Regex::new("(?<!\\\\)/\\*|\\*/|(?<!\\\\)\"|#\\[|\\]")
            .unwrap()
            .find_iter(&text)
            .filter_map(Result::ok)
            .collect();
        matches.sort_by_key(|m| m.start());

        let color_comment = QColor::from_rgb(103, 132, 181);
        let color_literal = QColor::from_rgb(111, 192, 244);
        let color_macro = QColor::from_rgb(176, 179, 11);

        #[derive(PartialEq, Debug)]
        enum State {
            Default,
            Comment,
            Literal,
            Macro,
        }

        let mut current_state = match self.as_mut().previous_block_state() {
            1 => State::Comment,
            2 => State::Literal,
            3 => State::Macro,
            _ => State::Default,
        };

        let mut highlight_start = 0;

        for mat in matches {
            let capture_length = mat.end() - highlight_start;

            current_state = match current_state {
                State::Default => match mat.as_str() {
                    "/*" => {
                        highlight_start = mat.start();
                        State::Comment
                    }
                    "\"" => {
                        highlight_start = mat.start();
                        State::Literal
                    }
                    "#[" => {
                        highlight_start = mat.start();
                        State::Macro
                    }
                    _ => State::Default,
                },

                State::Comment => {
                    if mat.as_str() == "*/" {
                        final_fmt.set_foreground(
                            highlight_start,
                            capture_length,
                            color_comment.clone(),
                        );
                        State::Default
                    } else {
                        State::Comment
                    }
                }

                State::Literal => {
                    if mat.as_str() == "\"" {
                        final_fmt.set_foreground(
                            highlight_start,
                            capture_length,
                            color_literal.clone(),
                        );

                        State::Default
                    } else {
                        State::Literal
                    }
                }

                State::Macro => {
                    if mat.as_str() == "]" {
                        final_fmt.set_foreground(
                            highlight_start,
                            capture_length,
                            color_macro.clone(),
                        );

                        State::Default
                    } else {
                        State::Macro
                    }
                }
            }
        }

        let color = match current_state {
            State::Default => {
                self.as_mut().set_current_block_state(0);
                color_comment
            }
            State::Comment => {
                self.as_mut().set_current_block_state(1);
                color_comment
            }
            State::Literal => {
                self.as_mut().set_current_block_state(2);
                color_literal
            }
            State::Macro => {
                self.as_mut().set_current_block_state(3);
                color_macro
            }
        };

        if current_state != State::Default {
            final_fmt.set_foreground(highlight_start, text.len() - highlight_start, color);
        }
    }
}
