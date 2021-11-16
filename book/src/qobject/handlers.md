<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Handlers

Handlers are used to react to events on the Qt event loop thread. This allows Rust to react to events from C++, process triggers from background Rust threads on the Qt foreground thread, and avoid deadlocks.

The following handlers are available

  * PropertyChangeHandler to handle when a property value has changed
  * UpdateRequestHandler to process update requests on the Qt event loop thread, see [threading](../concepts/threading.md) for more info.

## PropertyChangeHandler

When a property defined in the [data struct](./data_struct.md) is changed, either via Rust calling a setter or via QML / C++ calling a setter, we can be notified of this change by using the `PropertyChangeHandler`.

The example below listens to the number property and `handle_property_change` is triggered when the property `number` changes. It uses a `Property` enum which is automatically generated from the names of the properties defined in the [data struct](./data_struct.md).

Note that this is called from the Qt event loop thread.

```rust,ignore,noplayground
mod my_object {
    #[derive(Default)]
    struct Data {
        number: i32,
    }

    #[derive(Default)]
    struct RustObj;

    impl PropertyChangeHandler<CppObj<'_>, Property> for RustObj {
        fn handle_property_change(&mut self, cpp: &mut CppObj, property: Property) {
            match property {
                Property::Number => println!("New Number: {}", cpp.number()),
            }
        }
    }
}
```

## UpdateRequestHandler

When a background Rust thread uses an `UpdateRequester` to request the Qt thread to synchronise via calling `request_update` this triggers the `handle_update_request` method of the `UpdateRequestHandler`.

The snippet below shows a `handle_update_request` that when triggered iterates over an event_queue (which could be a channel from the background thread), to then update the values into the Qt object (via process_event with the CppObj).

Note that this is called from the Qt event loop thread.


```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_update_request_handler}}
```
