#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject *parent)
    : QObject(parent)
    , m_rustObj(create_my_object_rs())
{
}

MyObject::~MyObject() = default;

void MyObject::say_hi(const QString &string, int number) const
{
    auto rustString = rust::string(string.toUtf8().data(), bytes.length());
    m_rustObj->say_hi(std::move(rustString), number);
}

std::unique_ptr<MyObject> new_MyObject()
{
    return std::make_unique<MyObject>();
}
