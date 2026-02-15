#pragma once

#include <QWebEngineProfile>
#include <QWebEngineCookieStore>
#include <QWebEngineNotification>
#include <utility>

#include "rust/cxx.h"
#include "cxx-qt-lib/qstring.h"

using PersistentCookiesPolicy = QWebEngineProfile::PersistentCookiesPolicy;

namespace rust::cxxqtlib1 {
void notification_presenter_trampoline(
	const QWebEngineProfile* profile,
	std::unique_ptr<QWebEngineNotification> notification) noexcept;
} // namespace rust::cxxqtlib1

inline void setNotificationPresenter(QWebEngineProfile& profile) {
	profile.setNotificationPresenter(
		[profile_ptr = &profile](std::unique_ptr<QWebEngineNotification> notification) mutable {
			rust::cxxqtlib1::notification_presenter_trampoline(profile_ptr, std::move(notification));
		});
}

