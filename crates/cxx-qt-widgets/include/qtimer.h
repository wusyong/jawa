#pragma once

#include <QTimer>

#include <utility>

#include "rust/cxx.h"

template <typename Duration>
inline void singleShot(Duration interval, const QObject* context, rust::Fn<void()> functor) {
	QTimer::singleShot(
			interval,
			context,
			[functor = std::move(functor)]() mutable {
				functor();
			});
}

