# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# Add unit test for book
add_test(NAME book_build COMMAND mdbook build WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
add_test(NAME book_test COMMAND mdbook test WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
