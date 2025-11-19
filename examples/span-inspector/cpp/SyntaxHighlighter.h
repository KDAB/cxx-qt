// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <QColor>
#include <QQuickTextDocument>
#include <QRegularExpression>
#include <QSyntaxHighlighter>
#include <QTextCharFormat>
#include <QTextCursor>
#include <QVector>
#include <memory>

using MoveMode = QTextCursor::MoveMode;

struct HighlightRule
{
  QString regex;
  QColor color;
};

class SyntaxHighlighter : public QSyntaxHighlighter
{
  Q_OBJECT
public:
  explicit SyntaxHighlighter(QTextDocument* doc);
  void highlightBlock(const QString& text) override;
  bool renderError;
  void setRenderError(bool b) { renderError = b; }

private:
  std::vector<HighlightRule> highlightRules;
};