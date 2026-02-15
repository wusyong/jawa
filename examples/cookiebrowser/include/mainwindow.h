// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "ui_mainwindow.h"
#include "ui_cookiewidget.h"
#include "ui_cookiedialog.h"
#include <QDateTime>
#include <QMainWindow>
#include <QNetworkCookie>
#include <QPalette>
#include <QSpacerItem>
#include <QVBoxLayout>
#include <QWebEngineCookieStore>
#include <QWebEngineProfile>

QT_BEGIN_NAMESPACE
class QWebEngineCookieStore;
QT_END_NAMESPACE

class CookieDialog : public QDialog, public Ui_CookieDialog
{
    Q_OBJECT
public:
    CookieDialog(const QNetworkCookie &cookie, QWidget *parent = nullptr)
        : QDialog(parent)
    {
        setupUi(this);
        m_nameLineEdit->setText(cookie.name());
        m_domainLineEdit->setText(cookie.domain());
        m_valueLineEdit->setText(cookie.value());
        m_pathLineEdit->setText(cookie.path());
        m_dateEdit->setDate(cookie.expirationDate().date());
        m_isSecureComboBox->addItem(cookie.isSecure() ? tr("yes") : tr("no"));
        m_isHttpOnlyComboBox->addItem(cookie.isHttpOnly() ? tr("yes") : tr("no"));
        m_addButton->setVisible(false);
        m_cancelButton->setText(tr("Close"));
    }

    CookieDialog(QWidget *parent = 0)
        : QDialog(parent)
    {
        setupUi(this);
        m_nameLineEdit->setReadOnly(false);
        m_domainLineEdit->setReadOnly(false);
        m_valueLineEdit->setReadOnly(false);
        m_pathLineEdit->setReadOnly(false);
        m_dateEdit->setReadOnly(false);
        m_dateEdit->setDate(QDateTime::currentDateTime().addYears(1).date());
        m_isSecureComboBox->addItem(tr("no"));
        m_isSecureComboBox->addItem(tr("yes"));
        m_isHttpOnlyComboBox->addItem(tr("no"));
        m_isHttpOnlyComboBox->addItem(tr("yes"));
    }

    QNetworkCookie cookie()
    {
        QNetworkCookie cookie;
        cookie.setDomain(m_domainLineEdit->text());
        cookie.setName(m_nameLineEdit->text().toLatin1());
        cookie.setValue(m_valueLineEdit->text().toLatin1());
        cookie.setExpirationDate(QDateTime(m_dateEdit->date(), QTime::currentTime()));
        cookie.setPath(m_pathLineEdit->text());
        cookie.setSecure(m_isSecureComboBox->currentText() == tr("yes"));
        cookie.setHttpOnly(m_isHttpOnlyComboBox->currentText() == tr("yes"));
        return cookie;
    }
};

class CookieWidget : public QWidget,  public Ui_CookieWidget
{
    Q_OBJECT
public:
    CookieWidget(const QNetworkCookie &cookie, QWidget *parent = nullptr)
        : QWidget(parent)
    {
        setupUi(this);
        setAutoFillBackground(true);
        m_nameLabel->setText(cookie.name());
        m_domainLabel->setText(cookie.domain());
        connect(m_viewButton, &QPushButton::clicked, this, &CookieWidget::viewClicked);
        connect(m_deleteButton, &QPushButton::clicked, this, &CookieWidget::deleteClicked);
    }

    void setHighlighted(bool enabled)
    {
        m_isHighlighted = enabled;
        QPalette p = palette();
        p.setColor(backgroundRole(), enabled ? p.alternateBase().color() : p.base().color());
        setPalette(p);
    }
    bool isHighlighted() { return m_isHighlighted; }
signals:
    void deleteClicked();
    void viewClicked();

private:
    bool m_isHighlighted = false;
};

class MainWindow : public QMainWindow, public Ui_MainWindow
{
    Q_OBJECT
public:
    explicit MainWindow() {
        setupUi(this);
    }

    QLineEdit *urlLineEdit() const { return m_urlLineEdit; }
    // QScrollArea *scrollArea() const { return m_scrollArea; }
    QWidget *scrollArea() const { return m_scrollArea; }
    QPushButton *urlButton() const { return m_urlButton; }
    QPushButton *deleteAllButton() const { return m_deleteAllButton; }
    QPushButton *newButton() const { return m_newButton; }
    QWebEngineView *webview() const { return m_webview; }
    
    explicit MainWindow(const QUrl &url)
        : QMainWindow()
        , m_store(nullptr)
        , m_layout(new QVBoxLayout)
    {
        setupUi(this);
        m_urlLineEdit->setText(url.toString());

        m_layout->addItem(new QSpacerItem(0, 0, QSizePolicy::Minimum, QSizePolicy::Expanding));
        m_layout->setContentsMargins(0, 0, 0, 0);
        m_layout->setSpacing(0);

        QWidget *w = new QWidget();
        QPalette p = w->palette();
        p.setColor(widget->backgroundRole(), Qt::white);
        w->setPalette(p);
        w->setLayout(m_layout);

        m_scrollArea->setWidget(w);
        m_scrollArea->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        m_scrollArea->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOn);

        connect(m_urlButton, &QPushButton::clicked, this, &MainWindow::handleUrlClicked);
        connect(m_deleteAllButton, &QPushButton::clicked, this, &MainWindow::handleDeleteAllClicked);
        connect(m_newButton, &QPushButton::clicked, this, &MainWindow::handleNewClicked);

        m_store = m_webview->page()->profile()->cookieStore();
        connect(m_store, &QWebEngineCookieStore::cookieAdded, this, &MainWindow::handleCookieAdded);
        m_store->loadAllCookies();
        m_webview->load(url);
    }

private:
    bool containsCookie(const QNetworkCookie &cookie)
    {
        for (auto c : m_cookies) {
            if (c.hasSameIdentifier(cookie))
                return true;
        }
        return false;
    }

private slots:
    void handleCookieAdded(const QNetworkCookie &cookie)
    {
        // only new cookies
        if (containsCookie(cookie))
            return;

        CookieWidget *widget = new CookieWidget(cookie);
        m_cookies.append(cookie);
        // Check whether the first widget in the layout is highlighted.
        // if it is highlighted, then do not highlight the new item.
        CookieWidget *firstWidget = m_layout->count()
                ? qobject_cast<CookieWidget *>(m_layout->itemAt(0)->widget())
                : nullptr;
        if (firstWidget) {
            widget->setHighlighted(!firstWidget->isHighlighted());
        } else {
            widget->setHighlighted(false);
        }
        m_layout->insertWidget(0, widget);

        connect(widget, &CookieWidget::deleteClicked, [this, cookie, widget]() {
            m_store->deleteCookie(cookie);
            delete widget;
            m_cookies.removeOne(cookie);
            for (int i = 0; i < m_layout->count() - 1; i++) {
                // fix background colors
                auto widget = qobject_cast<CookieWidget*>(m_layout->itemAt(i)->widget());
                widget->setHighlighted(i % 2);
            }
        });

        connect(widget, &CookieWidget::viewClicked, [cookie]() {
            CookieDialog dialog(cookie);
            dialog.exec();
        });
    }

    void handleDeleteAllClicked()
    {
        m_store->deleteAllCookies();
        for (int i = m_layout->count() - 1; i >= 0; i--)
            delete m_layout->itemAt(i)->widget();
        m_cookies.clear();
    }

    void handleNewClicked()
    {
        CookieDialog dialog;
        if (dialog.exec() == QDialog::Accepted)
            m_store->setCookie(dialog.cookie());
    }

    void handleUrlClicked()
    {
        m_webview->load(QUrl::fromUserInput(m_urlLineEdit->text()));
    }

private:
    QWebEngineCookieStore *m_store;
    QList<QNetworkCookie> m_cookies;
    QVBoxLayout *m_layout;
};

#endif // MAINWINDOW_H
