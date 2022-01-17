mod my_object {
    #[derive(Default)]
    struct Data {
        color: Color,
        date: QDate,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        time: QTime,
        url: Url,
        variant: Variant,
    }

    #[derive(Default)]
    struct RustObj;
}
