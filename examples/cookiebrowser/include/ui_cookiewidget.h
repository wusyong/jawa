/********************************************************************************
** Form generated from reading UI file 'cookiewidget.ui'
**
** Created by: Qt User Interface Compiler version 6.10.1
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_COOKIEWIDGET_H
#define UI_COOKIEWIDGET_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QFormLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_CookieWidget
{
public:
    QHBoxLayout *horizontalLayout;
    QFormLayout *formLayout;
    QLabel *label;
    QLabel *m_nameLabel;
    QLabel *label_2;
    QLabel *m_domainLabel;
    QVBoxLayout *verticalLayout;
    QPushButton *m_viewButton;
    QPushButton *m_deleteButton;

    void setupUi(QWidget *CookieWidget)
    {
        if (CookieWidget->objectName().isEmpty())
            CookieWidget->setObjectName("CookieWidget");
        CookieWidget->resize(300, 84);
        QSizePolicy sizePolicy(QSizePolicy::Policy::Preferred, QSizePolicy::Policy::Preferred);
        sizePolicy.setHorizontalStretch(0);
        sizePolicy.setVerticalStretch(0);
        sizePolicy.setHeightForWidth(CookieWidget->sizePolicy().hasHeightForWidth());
        CookieWidget->setSizePolicy(sizePolicy);
        CookieWidget->setMinimumSize(QSize(300, 0));
        CookieWidget->setMaximumSize(QSize(310, 16777215));
        horizontalLayout = new QHBoxLayout(CookieWidget);
        horizontalLayout->setObjectName("horizontalLayout");
        formLayout = new QFormLayout();
        formLayout->setObjectName("formLayout");
        formLayout->setFormAlignment(Qt::AlignmentFlag::AlignLeading|Qt::AlignmentFlag::AlignLeft|Qt::AlignmentFlag::AlignVCenter);
        label = new QLabel(CookieWidget);
        label->setObjectName("label");

        formLayout->setWidget(0, QFormLayout::ItemRole::LabelRole, label);

        m_nameLabel = new QLabel(CookieWidget);
        m_nameLabel->setObjectName("m_nameLabel");

        formLayout->setWidget(0, QFormLayout::ItemRole::FieldRole, m_nameLabel);

        label_2 = new QLabel(CookieWidget);
        label_2->setObjectName("label_2");

        formLayout->setWidget(1, QFormLayout::ItemRole::LabelRole, label_2);

        m_domainLabel = new QLabel(CookieWidget);
        m_domainLabel->setObjectName("m_domainLabel");

        formLayout->setWidget(1, QFormLayout::ItemRole::FieldRole, m_domainLabel);


        horizontalLayout->addLayout(formLayout);

        verticalLayout = new QVBoxLayout();
        verticalLayout->setObjectName("verticalLayout");
        m_viewButton = new QPushButton(CookieWidget);
        m_viewButton->setObjectName("m_viewButton");

        verticalLayout->addWidget(m_viewButton);

        m_deleteButton = new QPushButton(CookieWidget);
        m_deleteButton->setObjectName("m_deleteButton");

        verticalLayout->addWidget(m_deleteButton);


        horizontalLayout->addLayout(verticalLayout);

        horizontalLayout->setStretch(0, 3);
        horizontalLayout->setStretch(1, 1);

        retranslateUi(CookieWidget);

        QMetaObject::connectSlotsByName(CookieWidget);
    } // setupUi

    void retranslateUi(QWidget *CookieWidget)
    {
        CookieWidget->setWindowTitle(QCoreApplication::translate("CookieWidget", "Form", nullptr));
        label->setText(QCoreApplication::translate("CookieWidget", "Name:", nullptr));
        m_nameLabel->setText(QCoreApplication::translate("CookieWidget", "Empty", nullptr));
        label_2->setText(QCoreApplication::translate("CookieWidget", "Domain:", nullptr));
        m_domainLabel->setText(QCoreApplication::translate("CookieWidget", "Emtpy", nullptr));
        m_viewButton->setText(QCoreApplication::translate("CookieWidget", "View", nullptr));
        m_deleteButton->setText(QCoreApplication::translate("CookieWidget", "Delete", nullptr));
    } // retranslateUi

};

namespace Ui {
    class CookieWidget: public Ui_CookieWidget {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_COOKIEWIDGET_H
