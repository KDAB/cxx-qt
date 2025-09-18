// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <QQuickTextDocument>
#include <QRegularExpression>
#include <QSyntaxHighlighter>
#include <QTextCharFormat>
#include <QVector>

class SyntaxHighlighter : public QSyntaxHighlighter
{
  Q_OBJECT
public:
  explicit SyntaxHighlighter(QTextDocument* doc);
  void highlightBlock(const QString& text) override;
};

std::unique_ptr<SyntaxHighlighter>
new_syntax_highlighter(QQuickTextDocument*);