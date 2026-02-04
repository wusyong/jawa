#pragma once

#include <QWebEngineView>

#include "rust/cxx.h"

#include <cstdint>

namespace rust {

// QWebEnginePermission has a move constructor, however it is basically trivial.
template<>
struct IsRelocatable<QWebEnginePermission> : ::std::true_type
{};

} // namespace rust
