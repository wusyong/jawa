#pragma once

#include <QPalette>
#include <QWidget>
#include <QColor>

#include <memory>
#include "rust/cxx.h"

using ColorRole = ::QPalette::ColorRole;

inline void qpaletteSetColor(QPalette& palette, ColorRole role, const QColor& color)
{
  palette.setColor(role, color);
}

inline QColor qpaletteColor(const QPalette& palette, ColorRole role)
{
  return palette.color(role);
}

namespace rust {

template<>
struct IsRelocatable<ColorRole> : ::std::true_type
{};

} // namespace rust
