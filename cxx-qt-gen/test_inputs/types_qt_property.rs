mod my_object {
    #[derive(Default)]
    struct Data {
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        variant: Variant,
    }

    #[derive(Default)]
    struct RustObj;
}
