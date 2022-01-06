mod my_object {
    #[derive(Default)]
    struct Data {
        point: QPoint,
        pointf: QPointF,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        variant: Variant,
    }

    #[derive(Default)]
    struct RustObj;
}
