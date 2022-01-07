mod my_object {
    #[derive(Default)]
    struct Data {
        point: QPoint,
        pointf: QPointF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        variant: Variant,
    }

    #[derive(Default)]
    struct RustObj;
}
