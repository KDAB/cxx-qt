mod my_object {
    #[derive(Default)]
    struct Data {
        color: Color,
        date: QDate,
        date_time: DateTime,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        time: QTime,
        url: Url,
        variant: QVariant,
    }

    #[derive(Default)]
    struct RustObj;
}
