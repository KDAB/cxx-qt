// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/include/qt_types.h"

namespace rust {
namespace cxxqtlib1 {

std::unique_ptr<QColor>
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a)
{
  return std::make_unique<QColor>(r, g, b, a);
}

std::unique_ptr<QColor>
qcolorInitFromQColor(const QColor& color)
{
  return std::make_unique<QColor>(color);
}

QDate
qdateInitDefault()
{
  return QDate();
}

QDate
qdateInit(int y, int m, int d)
{
  return QDate(y, m, d);
}

QPoint
qpointInitDefault()
{
  return QPoint();
}

QPoint
qpointInit(int x, int y)
{
  return QPoint(x, y);
}

QPointF
qpointfInitDefault()
{
  return QPointF();
}

QPointF
qpointfInit(qreal x, qreal y)
{
  return QPointF(x, y);
}

QRect
qrectInitDefault()
{
  return QRect();
}

QRect
qrectInit(int x, int y, int w, int h)
{
  return QRect(x, y, w, h);
}

QRectF
qrectfInitDefault()
{
  return QRectF();
}

QRectF
qrectfInit(qreal x, qreal y, qreal w, qreal h)
{
  return QRectF(x, y, w, h);
}

QSize
qsizeInitDefault()
{
  return QSize();
}

QSize
qsizeInit(int width, int height)
{
  return QSize(width, height);
}

QSizeF
qsizefInitDefault()
{
  return QSizeF();
}

QSizeF
qsizefInit(qreal width, qreal height)
{
  return QSizeF(width, height);
}

std::unique_ptr<QString>
qstringInitFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return std::make_unique<QString>(
    QString::fromStdString(static_cast<std::string>(string)));
}

rust::String
qstringToRustString(const QString& string)
{
  // Note that this changes UTF-16 to UTF-8
  return rust::String(string.toStdString());
}

QTime
qtimeInitDefault()
{
  return QTime();
}

QTime
qtimeInit(int h, int m, int s, int ms)
{
  return QTime(h, m, s, ms);
}

}
}
