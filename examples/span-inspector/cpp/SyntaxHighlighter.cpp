// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "SyntaxHighlighter.h"

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
      { "\".*\"|(?<=fn\\s)\\w*|", "#6fc0f4" },
      { "//.*", "#6784b5" } });
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

  QRegularExpression regCommentStart("/\\*");
  QRegularExpression regCommentEnd("\\*/");

  QRegularExpressionMatchIterator startMatches =
    QRegularExpression(regCommentStart).globalMatch(text);
  QRegularExpressionMatchIterator endMatches =
    QRegularExpression(regCommentEnd).globalMatch(text);

  QTextCharFormat charFormat;
  charFormat.setForeground(QColor("#6784b5"));

  setCurrentBlockState(-1);

  if (previousBlockState() == 0) {
    if (endMatches.hasNext()) {
      QRegularExpressionMatch match = endMatches.next();
      setFormat(0, match.capturedEnd(), charFormat);
    } else {
      setFormat(0, text.length(), charFormat);
      setCurrentBlockState(0);
    }
  }

  QRegularExpressionMatch start;
  QRegularExpressionMatch end;

  while (startMatches.hasNext()) {
    start = startMatches.next();

    if (endMatches.hasNext()) {
      end = endMatches.next();
    } else {
      setCurrentBlockState(0);
    }

    while (start.capturedStart() >= end.capturedEnd()) {
      if (!startMatches.hasNext()) {
        break;
      }
      startMatches.next();
    }

    int endPos = (currentBlockState() == 0) ? text.length() : end.capturedEnd();

    setFormat(
      start.capturedStart(), endPos - start.capturedStart(), charFormat);
  }
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