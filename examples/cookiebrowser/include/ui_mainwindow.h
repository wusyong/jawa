/********************************************************************************
** Form generated from reading UI file 'mainwindow.ui'
**
** Created by: Qt User Interface Compiler version 6.10.1
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAINWINDOW_H
#define UI_MAINWINDOW_H

#include <QtCore/QVariant>
#include <QtGui/QIcon>
#include <QtWebEngineWidgets/QWebEngineView>
#include <QtWidgets/QApplication>
#include <QtWidgets/QFrame>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QScrollArea>
#include <QtWidgets/QSpacerItem>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralWidget;
    QHBoxLayout *horizontalLayout_2;
    QFrame *frame;
    QVBoxLayout *verticalLayout;
    QWidget *widget;
    QHBoxLayout *horizontalLayout;
    QLineEdit *m_urlLineEdit;
    QPushButton *m_urlButton;
    QWebEngineView *m_webview;
    QFrame *frame_2;
    QVBoxLayout *verticalLayout_2;
    QWidget *widget_2;
    QHBoxLayout *horizontalLayout_3;
    QLabel *label;
    QSpacerItem *horizontalSpacer;
    QPushButton *m_newButton;
    QPushButton *m_deleteAllButton;
    QScrollArea *m_scrollArea;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName("MainWindow");
        MainWindow->resize(1400, 650);
        centralWidget = new QWidget(MainWindow);
        centralWidget->setObjectName("centralWidget");
        horizontalLayout_2 = new QHBoxLayout(centralWidget);
        horizontalLayout_2->setSpacing(6);
        horizontalLayout_2->setContentsMargins(11, 11, 11, 11);
        horizontalLayout_2->setObjectName("horizontalLayout_2");
        frame = new QFrame(centralWidget);
        frame->setObjectName("frame");
        frame->setFrameShape(QFrame::Shape::StyledPanel);
        frame->setFrameShadow(QFrame::Shadow::Raised);
        verticalLayout = new QVBoxLayout(frame);
        verticalLayout->setSpacing(6);
        verticalLayout->setContentsMargins(11, 11, 11, 11);
        verticalLayout->setObjectName("verticalLayout");
        widget = new QWidget(frame);
        widget->setObjectName("widget");
        horizontalLayout = new QHBoxLayout(widget);
        horizontalLayout->setSpacing(6);
        horizontalLayout->setContentsMargins(11, 11, 11, 11);
        horizontalLayout->setObjectName("horizontalLayout");
        horizontalLayout->setContentsMargins(0, 0, 0, 0);
        m_urlLineEdit = new QLineEdit(widget);
        m_urlLineEdit->setObjectName("m_urlLineEdit");

        horizontalLayout->addWidget(m_urlLineEdit);

        m_urlButton = new QPushButton(widget);
        m_urlButton->setObjectName("m_urlButton");
        QIcon icon;
        icon.addFile(QString::fromUtf8(":/view-refresh.png"), QSize(), QIcon::Mode::Normal, QIcon::State::Off);
        m_urlButton->setIcon(icon);

        horizontalLayout->addWidget(m_urlButton);


        verticalLayout->addWidget(widget);

        m_webview = new QWebEngineView(frame);
        m_webview->setObjectName("m_webview");
        QSizePolicy sizePolicy(QSizePolicy::Policy::Preferred, QSizePolicy::Policy::Expanding);
        sizePolicy.setHorizontalStretch(0);
        sizePolicy.setVerticalStretch(0);
        sizePolicy.setHeightForWidth(m_webview->sizePolicy().hasHeightForWidth());
        m_webview->setSizePolicy(sizePolicy);
        m_webview->setMinimumSize(QSize(0, 0));

        verticalLayout->addWidget(m_webview);


        horizontalLayout_2->addWidget(frame);

        frame_2 = new QFrame(centralWidget);
        frame_2->setObjectName("frame_2");
        frame_2->setMaximumSize(QSize(336, 16777215));
        frame_2->setFrameShape(QFrame::Shape::StyledPanel);
        frame_2->setFrameShadow(QFrame::Shadow::Raised);
        verticalLayout_2 = new QVBoxLayout(frame_2);
        verticalLayout_2->setSpacing(6);
        verticalLayout_2->setContentsMargins(11, 11, 11, 11);
        verticalLayout_2->setObjectName("verticalLayout_2");
        widget_2 = new QWidget(frame_2);
        widget_2->setObjectName("widget_2");
        horizontalLayout_3 = new QHBoxLayout(widget_2);
        horizontalLayout_3->setSpacing(6);
        horizontalLayout_3->setContentsMargins(11, 11, 11, 11);
        horizontalLayout_3->setObjectName("horizontalLayout_3");
        horizontalLayout_3->setContentsMargins(0, 0, 0, 0);
        label = new QLabel(widget_2);
        label->setObjectName("label");

        horizontalLayout_3->addWidget(label);

        horizontalSpacer = new QSpacerItem(87, 20, QSizePolicy::Policy::Expanding, QSizePolicy::Policy::Minimum);

        horizontalLayout_3->addItem(horizontalSpacer);

        m_newButton = new QPushButton(widget_2);
        m_newButton->setObjectName("m_newButton");

        horizontalLayout_3->addWidget(m_newButton);

        m_deleteAllButton = new QPushButton(widget_2);
        m_deleteAllButton->setObjectName("m_deleteAllButton");

        horizontalLayout_3->addWidget(m_deleteAllButton);


        verticalLayout_2->addWidget(widget_2);

        m_scrollArea = new QScrollArea(frame_2);
        m_scrollArea->setObjectName("m_scrollArea");
        m_scrollArea->setMinimumSize(QSize(320, 0));
        m_scrollArea->setMaximumSize(QSize(320, 16777215));
        m_scrollArea->setWidgetResizable(true);

        verticalLayout_2->addWidget(m_scrollArea);


        horizontalLayout_2->addWidget(frame_2);

        MainWindow->setCentralWidget(centralWidget);

        retranslateUi(MainWindow);
        QObject::connect(m_urlLineEdit, &QLineEdit::returnPressed, m_urlButton, qOverload<>(&QPushButton::click));

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QCoreApplication::translate("MainWindow", "Cookie Manager", nullptr));
        m_urlButton->setText(QString());
        label->setText(QCoreApplication::translate("MainWindow", "Cookies:", nullptr));
        m_newButton->setText(QCoreApplication::translate("MainWindow", "New", nullptr));
        m_deleteAllButton->setText(QCoreApplication::translate("MainWindow", "Delete All", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
