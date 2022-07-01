<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Handlers

Handlers are used to react to events on the Qt event loop thread. This allows Rust to react to events from C++, process triggers from background Rust threads on the Qt foreground thread, and avoid deadlocks.

The following handlers are available

  * UpdateRequestHandler to process update requests on the Qt event loop thread, see [threading](../concepts/threading.md) for more info.

## UpdateRequestHandler

When a background Rust thread uses an `UpdateRequester` to request the Qt thread to synchronise via calling `request_update` this triggers the `handle_update_request` method of the `UpdateRequestHandler`.

For example in an invokable the [`CppObj`](./cpp_object.md) is used to retrieve an `UpdateRequester`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_cpp_update_requester}}
```

The `UpdateRequester` is moved into the thread, then when required an update is requested.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_request_update}}
```

This then triggers `handle_update_request` to be called at a later stage from the Qt event loop thread. Which can iterate over an event_queue (eg a channel from the background thread), to update the values into the Qt object (via process_event with the CppObj).

Note that this is called from the Qt event loop thread.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_update_request_handler}}
```
