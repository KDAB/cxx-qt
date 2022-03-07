// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QPoint>

namespace rust {
namespace cxxqtlib1 {

QPoint
qpointInitDefault();
QPoint
qpointInit(int x, int y);

}
}
