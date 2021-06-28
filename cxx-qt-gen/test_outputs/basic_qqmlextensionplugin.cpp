#include <QQmlEngine>
#include <QQmlExtensionPlugin>

#include "cxx-qt-gen/include/my_object.h"

class CppPluginNamePlugin : public QQmlExtensionPlugin
{
  Q_OBJECT
  Q_PLUGIN_METADATA(IID QQmlExtensionInterface_iid)

public:
  void registerTypes(const char* uri) override
  {
    qmlRegisterType<MyObject>(uri, 1, 0, "MyObject");
  }
};

#include "plugin.moc"
