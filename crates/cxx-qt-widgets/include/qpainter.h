#pragma once

#include <QPainter>
#include <QPrinter>

#include "rust/cxx.h"

// Trampolins for QPainter
namespace rust::cxxqtlib1
{
	inline bool qpainter_begin(QPainter &painter, QPrinter &device) noexcept
	{
		return painter.begin(&device);
	}

	inline bool qpainter_end(QPainter &painter) noexcept
	{
		return painter.end();
	}

	inline void qpainter_draw_text(QPainter &painter, const QPointF &position, const QString &text) noexcept
	{
		painter.drawText(position, text);
	}
} // namespace rust::cxxqtlib1
