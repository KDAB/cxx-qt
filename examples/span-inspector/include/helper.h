// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <QSyntaxHighlighter>

class QSyntaxHighlighterCXX : public QSyntaxHighlighter
{
  Q_OBJECT
public:
  explicit QSyntaxHighlighterCXX(QTextDocument* doc)
    : QSyntaxHighlighter(doc)
  {
  }

protected:
  std::unique_ptr<QTextBlock> currentBlockCXX()
  {
    return std::unique_ptr<QTextBlock>(new QTextBlock(currentBlock()));
  }
};