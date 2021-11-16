<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Signals enum

The signals enum defines which signals should exist on the QObject. It allows you to define the signal name and the parameters of the signal.

TODO: use real example once we have the code

```rust,ignore,noplayground
#[make_qobject]
mod my_object {
    enum Signals {
        Ready,
        DataChanged { data: i32 },
    }
}
```

Note: signals are not implemented yet [https://github.com/KDAB/cxx-qt/issues/31](https://github.com/KDAB/cxx-qt/issues/31).
## Emitting a signal

To emit a signal from Rust use the [`CppObj`](./cpp_object.md) and call either the `emit_queued(Signal)` or `unsafe emit_immediate(Signal)` method.

TODO: use real example once we have the code

```rust,ignore,noplayground
impl RustObj {
    #[invokable]
    fn invokable(&self, cpp: &mut CppObj) {
        unsafe { cpp.emit_immediate(Signal::Ready); }

        cpp.emit_queued(Signal::DataChanged { data: 1 });
    }
}
```

Note: signals are not implemented yet [https://github.com/KDAB/cxx-qt/issues/31](https://github.com/KDAB/cxx-qt/issues/31).
