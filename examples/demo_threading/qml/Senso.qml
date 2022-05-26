import QtQuick 2.12
import QtQuick.Window 2.12
import com.kdab.energy 1.0

Item {
    id: senso
    //property var  model: sensorModel
    property string uuid: "foo1"
    property double power: t1.np * online
    property bool online: false
    property int type: 0
    property int max: 3500
    property int min: 1000
    Timer {
        id: t1
        property int np: 0
        interval: (6000+(500 * Math.random()))
        running: true
        onTriggered: np = Math.ceil((max-min) * Math.random()+min)
        repeat: true
        triggeredOnStart: true
    }
    Timer {
        interval: (5400+60000 * Math.random())
        onTriggered: online = Math.random()>0.2
        repeat: true
        running: true
        triggeredOnStart: true
    }
}
