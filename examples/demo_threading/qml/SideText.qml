import QtQuick 2.12
import QtQuick.Window 2.12
import com.kdab.energy 1.0

Text {
    id:sideText

    font.family: "Open Sans"
    font.italic: true
    font.pixelSize: 12
    Behavior on scale {  NumberAnimation {
            duration: 570
            easing.type: Easing.OutQuad
        }}
    color: "#a9deff"
    font.weight: Font.Light
    transformOrigin: Item.Left
    height: scale*paintedHeight
    opacity: scale*scale - 0.5

}
