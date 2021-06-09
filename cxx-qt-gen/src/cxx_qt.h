#pragma once

#include <memory>

#include <QObject>
#include <QString>

#include "rust/cxx.h"

inline rust::string
qStringToRustString(const QString& value)
{
  const auto bytes = value.toUtf8();
  return rust::string(bytes.data(), bytes.length());
}

inline QString
rustStringToQString(const rust::string& value)
{
  return QString::fromUtf8(value.data(), value.length());
}
