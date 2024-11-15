// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// When building with no exceptions ensure we do not have a try catch
// error: cannot use 'try' with exceptions disabled
// So override the default trycatch from CXX
#if defined(RUST_CXX_NO_EXCEPTIONS)
namespace rust {
namespace behavior {

template<typename Try, typename Fail>
static void
trycatch(Try&& func, Fail&& fail) noexcept
{
  func();
}

} // namespace behavior
} // namespace rust
#endif
