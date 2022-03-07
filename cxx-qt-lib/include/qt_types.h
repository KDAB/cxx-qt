// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <memory>

#include <QColor>
#include <QPoint>
#include <QPointF>
#include <QRect>
#include <QRectF>
#include <QString>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

std::unique_ptr<QColor>
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a);
std::unique_ptr<QColor>
qcolorInitFromQColor(const QColor& color);

QPoint
qpointInitDefault();
QPoint
qpointInit(int x, int y);

QPointF
qpointfInitDefault();
QPointF
qpointfInit(qreal x, qreal y);

QRect
qrectInitDefault();
QRect
qrectInit(int x, int y, int w, int h);

QRectF
qrectfInitDefault();
QRectF
qrectfInit(qreal x, qreal y, qreal w, qreal h);

std::unique_ptr<QString>
qstringInitFromRustString(rust::Str string);
rust::String
qstringToRustString(const QString& string);

}
}
