mod my_object {
    #[derive(Default)]
    pub struct Data {
        color: QColor,
        date: QDate,
        date_time: QDateTime,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        string: String,
        time: QTime,
        url: QUrl,
        variant: QVariant,
    }

    #[derive(Default)]
    pub struct RustObj;
}
