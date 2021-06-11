#include <QtCore/QtGlobal>
#include <QtQml/QQmlEngine>
#include <QtQuickTest/quicktest.h>

#include "cxx-qt-gen/include/my_object.h"

// TODO: Once we can make a QQmlExtensionPlugin we won't need this C++ helper
// as we'll be able to import the .so + qmldir from QML directly
class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");
  }
};

QUICK_TEST_MAIN_WITH_SETUP(myobject, Setup)

#include "tst_myobject.moc"
