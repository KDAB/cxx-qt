import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

import com.kdab.energy 1.0

QtObject {
    id: senso
    property var  model: sensorModel
    property string uuid: "foo1"
    property double power: 0
    property bool on: off

}
