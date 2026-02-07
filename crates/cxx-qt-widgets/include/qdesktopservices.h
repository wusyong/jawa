#pragma once

#include <QDesktopServices>

#include "rust/cxx.h"
#include "cxx-qt-lib/qobject.h"
#include "cxx-qt-lib/qurl.h"
#include "cxx-qt-lib/qstring.h"

#include <string>

inline bool open_url(const QUrl& url) {
  return QDesktopServices::openUrl(url);
}

inline void set_url_handler(const QString& scheme, QObject* receiver, rust::Str method) {
  std::string method_str(method.data(), method.size());
  QDesktopServices::setUrlHandler(scheme, receiver, method_str.c_str());
}

inline void unset_url_handler(const QString& scheme) {
  QDesktopServices::unsetUrlHandler(scheme);
}