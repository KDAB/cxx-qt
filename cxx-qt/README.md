<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# cxx-qt

cxx-qt is a library that automatically generates code to transfer data between Rust and C++ through common interfaces
such as QObjects that can be exposed directly into QML. It relies on the cxx crate internally to achieve this and thus
it is recommended that any interactions with Qt that are not covered by the built-in code generators should be done
directly in C++ and connected to relevant Rust logic by writing additional cxx code. The cxx-qt build system is based
on CMake, but is compatible with cxx on its own as well.
