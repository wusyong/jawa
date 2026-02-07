#pragma once

#include <QWebEnginePermission>

#include "rust/cxx.h"
#include "cxx-qt-lib/qurl.h"

#include <cstdint>
#include <cxx-qt-lib/assertion_utils.h>

using PermissionType = QWebEnginePermission::PermissionType;
using State = QWebEnginePermission::State;

assert_alignment_and_size(QWebEnginePermission, {
  ::std::size_t a1;
});

namespace rust {

// QWebEnginePermission has a move constructor, however it is basically trivial.
template<>
struct IsRelocatable<QWebEnginePermission> : ::std::true_type
{};

} // namespace rust
