// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::pin::Pin;

use cxx_qt_lib::QString;
use qobject::TodoRoles;

#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!(< QAbstractListModel >);
        type QAbstractListModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
    }

    #[qenum(TodoList)]
    enum TodoRoles {
        Todo,
        Done,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        type TodoList = super::TodoListRust;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(&self, parent: &QModelIndex) -> i32;

        #[cxx_override]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(&self) -> QHash_i32_QByteArray;

        #[qinvokable]
        #[rust_name = "set_checked"]
        fn setChecked(self: Pin<&mut Self>, row: i32, checked: bool);

        #[inherit]
        #[rust_name = "begin_reset_model"]
        fn beginResetModel(self: Pin<&mut Self>);

        #[inherit]
        #[rust_name = "end_reset_model"]
        fn endResetModel(self: Pin<&mut Self>);

        #[qinvokable]
        #[rust_name = "add_todo"]
        fn addTodo(self: Pin<&mut Self>, todo: &QString);
    }
}

pub struct TodoListRust {
    todos: Vec<(bool, QString)>,
}

impl Default for TodoListRust {
    fn default() -> Self {
        Self {
            todos: vec![
                (true, "Build ToDo Example".into()),
                (false, "Modify ToDo Example".into()),
            ],
        }
    }
}

use cxx_qt::CxxQtType;
use qobject::*;

impl qobject::TodoList {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.todos.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = TodoRoles { repr: role };

        if let Some((done, ref todo)) = self.todos.get(index.row() as usize) {
            match role {
                TodoRoles::Todo => {
                    return todo.into();
                }
                TodoRoles::Done => {
                    return done.into();
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(TodoRoles::Todo.repr, "todo".into());
        hash.insert(TodoRoles::Done.repr, "done".into());
        hash
    }

    fn set_checked(mut self: Pin<&mut Self>, row: i32, checked: bool) {
        if let Some((done, _todo)) = self.as_mut().rust_mut().todos.get_mut(row as usize) {
            if *done != checked {
                *done = checked;
                // self.sort() will reset the model, so we don't need to emit dataChanged here.
                self.sort();
            }
        }
    }

    fn sort(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();
        self.as_mut()
            .rust_mut()
            .todos
            .sort_by_key(|(done, _todo)| *done);
        self.as_mut().end_reset_model();
    }

    fn add_todo(mut self: Pin<&mut Self>, todo: &QString) {
        self.as_mut().rust_mut().todos.push((false, todo.clone()));
        // self.sort() will reset the model, so we don't need to emit begin/endInsertRows here
        self.sort();
    }
}
