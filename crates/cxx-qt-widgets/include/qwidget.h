#pragma once

#include <QObject>
#include <QWidget>
#include "cxx-qt-widgets/qpalette.h"

#include <memory>

#include "rust/cxx.h"

class RustQWidget : public QWidget {
  Q_OBJECT
  public:
   explicit RustQWidget(QObject* parent = nullptr)
      : QWidget(qobject_cast<QWidget*>(parent)) {}
};
