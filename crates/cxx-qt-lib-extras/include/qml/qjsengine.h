#pragma once

#include <memory>

#include <QtQml/QJSEngine>

#include "rust/cxx.h"

namespace rust
{
    namespace cxxqtlib2
    {
        ::std::unique_ptr<QJSEngine>
        qjsengineNew();

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineNewArray(T &engine, quint32 length)
        {
            auto ptr = ::std::make_unique<QJSValue>(engine.newArray(length));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineNewObject(T &engine)
        {
            auto ptr = ::std::make_unique<QJSValue>(engine.newObject());
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineNewQObject(T &engine, QObject &object)
        {
            auto ptr = ::std::make_unique<QJSValue>(engine.newQObject(&object));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineEvaluate(
            T &engine,
            const QString &program,
            const QString &fileName,
            int lineNumber)
        {
            auto ptr = ::std::make_unique<QJSValue>(
                engine.evaluate(program, fileName, lineNumber, nullptr));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineImportModule(T &engine, const QString &fileName)
        {
            auto ptr = ::std::make_unique<QJSValue>(engine.importModule(fileName));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        template <typename T>
        ::std::unique_ptr<QJSValue>
        jsengineGlobalObject(T &engine)
        {
            auto ptr = ::std::make_unique<QJSValue>(engine.globalObject());
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }
    }
}
