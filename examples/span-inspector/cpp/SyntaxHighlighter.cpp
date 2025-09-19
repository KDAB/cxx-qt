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
  highlightRules.assign(
    { { "\\w*::|None|Some|\\d", "#f99853" },
      { "(?<!\\w)(use|struct|pub|impl|fn|Self|if|let|else|ref|"
        "mut|while|for|in|extern|"
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

  for (QRegularExpressionMatch match :
       QRegularExpression("(?<!\\\\)/\\*|\\*/|(?<!\\\\)\"").globalMatch(text)) {
    matches.push_back(match);
  }

  std::sort(matches.begin(), matches.end(), [](const auto& a, const auto& b) {
    return a.capturedStart() < b.capturedStart();
  });

  QTextCharFormat charFormatComment, charFormatLiteral;
  charFormatComment.setForeground(QColor("#6784b5"));
  charFormatLiteral.setForeground(QColor("#6fc0f4"));

  enum class State
  {
    Default,
    Comment,
    Literal
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
    }
  }

  switch (currentState) {
    case State::Comment:
      setFormat(
        highlightStart, text.length() - highlightStart, charFormatComment);
      break;

    case State::Literal:
      setFormat(
        highlightStart, text.length() - highlightStart, charFormatLiteral);
      break;
    case State::Default:
      break;
  }

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