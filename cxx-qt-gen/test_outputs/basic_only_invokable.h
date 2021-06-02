#pragma once

#include <QObject>
#include <QString>

#include "rust/cxx.h"

class MyObjectRs;

class MyObject : public QObject {
    Q_OBJECT

public:
    MyObject(QObject *parent = nullptr);
    ~MyObject();

    Q_INVOKABLE void say_hi(const QString &string, int number) const;

private:
    rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject> new_MyObject();
