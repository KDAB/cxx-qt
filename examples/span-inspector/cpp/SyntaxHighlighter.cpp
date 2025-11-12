// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "SyntaxHighlighter.h"
#include <deque>

SyntaxHighlighter::SyntaxHighlighter(QTextDocument* doc)
  : QSyntaxHighlighter(doc)
{
  renderError = false;
  highlightRules.assign(
    { { "\\w*::|None|Some|\\d", "#f99853" },
      { "(?<!\\w)(use|struct|pub|impl|fn|Self|if|let|else|ref|"
        "mut|while|for|in|extern|type|unsafe|"
        "crate|match|loop|break|str|mod|usize|isize|char|bool|(u|i|f)\\d{1, "
        "2})(?!\\w)",
        "#ff7b72" },
      { "\\->|=>|\\+=|-=|!|&|=|<|>|&|\\*", "#407ecf" },
      { "(?<=fn\\s)\\w*|", "#6fc0f4" },
      { "//.*", "#6784b5" } });
  setCurrentBlockState(0);
}

void
SyntaxHighlighter::highlightBlock(const QString& text)
{
  if (renderError)
    return;

  for (HighlightRule rule : highlightRules) {
    QTextCharFormat charFormat;
    charFormat.setForeground(rule.color);

    QRegularExpression expression(rule.regex);
    QRegularExpressionMatchIterator i = expression.globalMatch(text);
    while (i.hasNext()) {
      QRegularExpressionMatch match = i.next();
      setFormat(match.capturedStart(), match.capturedLength(), charFormat);
    }
  }

  std::deque<QRegularExpressionMatch> matches;

  //                              /*     | */ |     "     | #[ | ]
  QRegularExpression regex("(?<!\\\\)/\\*|\\*/|(?<!\\\\)\"|#\\[|\\]");
  for (QRegularExpressionMatch match : regex.globalMatch(text)) {
    matches.push_back(match);
  }

  std::sort(matches.begin(), matches.end(), [](const auto& a, const auto& b) {
    return a.capturedStart() < b.capturedStart();
  });

  QTextCharFormat charFormatComment, charFormatLiteral, charFormatMacro;
  charFormatComment.setForeground(QColor("#6784b5"));
  charFormatLiteral.setForeground(QColor("#6fc0f4"));
  charFormatMacro.setForeground(QColor("#b0b30b"));

  enum class State
  {
    Default,
    Comment,
    Literal,
    Macro,
  };

  State currentState = (previousBlockState() == -1)
                         ? State::Default
                         : static_cast<State>(previousBlockState());

  int highlightStart = 0;

  while (!matches.empty()) {
    const QRegularExpressionMatch match = matches.front();
    matches.pop_front();

    const QString capture = match.captured(0);
    int capturedStart = match.capturedStart();
    int capturedEnd = match.capturedEnd();

    switch (currentState) {
      case State::Default:
        if (capture == "/*") {
          currentState = State::Comment;
          highlightStart = capturedStart;
        } else if (capture == "\"") {
          currentState = State::Literal;
          highlightStart = capturedStart;
        } else if (capture == "#[") {
          currentState = State::Macro;
          highlightStart = capturedStart;
        }
        break;

      case State::Comment:
        if (capture == "*/") {
          currentState = State::Default;
          setFormat(
            highlightStart, capturedEnd - highlightStart, charFormatComment);
        }
        break;

      case State::Literal:
        if (capture == "\"") {
          currentState = State::Default;
          setFormat(
            highlightStart, capturedEnd - highlightStart, charFormatLiteral);
        }
        break;

      case State::Macro:
        if (capture == "]") {
          currentState = State::Default;
          setFormat(
            highlightStart, capturedEnd - highlightStart, charFormatMacro);
        }
        break;
    }
  }

  QTextCharFormat charFormat;

  if (currentState == State::Comment) {
    charFormat = charFormatComment;
  } else if (currentState == State::Literal) {
    charFormat = charFormatLiteral;
  } else if (currentState == State::Macro) {
    charFormat = charFormatMacro;
  }

  if (!(currentState == State::Default))
    setFormat(highlightStart, text.length() - highlightStart, charFormat);

  setCurrentBlockState(static_cast<int>(currentState));
}

std::unique_ptr<SyntaxHighlighter>
new_syntax_highlighter(QQuickTextDocument* text_document)
{
  if (text_document == NULL) {
    return NULL;
  }
  return std::unique_ptr<SyntaxHighlighter>(
    new SyntaxHighlighter(text_document->textDocument()));
}

std::unique_ptr<QTextCursor>
new_QTextCursor(QQuickTextDocument* text_document)
{
  if (text_document == NULL) {
    return NULL;
  }
  return std::unique_ptr<QTextCursor>(
    new QTextCursor(text_document->textDocument()));
}

std::unique_ptr<QTextCharFormat>
new_QTextCharFormat()
{
  return std::unique_ptr<QTextCharFormat>(new QTextCharFormat());
}

std::unique_ptr<QBrush>
new_QBrush(const QColor& color)
{
  return std::unique_ptr<QBrush>(new QBrush(color));
}