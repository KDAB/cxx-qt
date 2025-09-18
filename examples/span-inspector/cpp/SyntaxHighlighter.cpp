// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "SyntaxHighlighter.h"
#include <QQuickTextDocument>
#include <QRegularExpression>
#include <QSyntaxHighlighter>
#include <QTextCharFormat>

SyntaxHighlighter::SyntaxHighlighter(QTextDocument* doc)
  : QSyntaxHighlighter(doc)
{
}

void
SyntaxHighlighter::highlightBlock(const QString& text)
{
  QTextCharFormat myClassFormat;
  myClassFormat.setFontWeight(QFont::Bold);
  myClassFormat.setForeground(Qt::darkMagenta);

  QRegularExpression expression("mod");
  QRegularExpressionMatchIterator i = expression.globalMatch(text);
  while (i.hasNext()) {
    QRegularExpressionMatch match = i.next();
    setFormat(match.capturedStart(), match.capturedLength(), myClassFormat);
  }
}

std::unique_ptr<SyntaxHighlighter>
new_syntax_highlighter(QQuickTextDocument* text_document)
{
  if (text_document == NULL) {
    printf("null");
    return NULL;
  }
  return std::unique_ptr<SyntaxHighlighter>(
    new SyntaxHighlighter(text_document->textDocument()));
}