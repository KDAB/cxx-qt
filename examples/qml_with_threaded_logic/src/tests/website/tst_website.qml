// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "WebsiteTests"

    Component {
        id: componentWebsite

        Website {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_defaults() {
        const website = createTemporaryObject(componentWebsite, null, {});
        compare(website.url, "known");
        compare(website.title, "Press refresh to get a title...");
    }

    function test_title_refresh() {
        const website = createTemporaryObject(componentWebsite, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "titleChanged",
            target: website,
        });

        compare(spy.count, 0);

        website.refreshTitle();

        spy.wait();
        compare(website.title, "Loading...");
        compare(spy.count, 1);

        spy.wait();
        compare(website.title, "Known website");
        compare(spy.count, 2);
    }

    function test_url_change() {
        const website = createTemporaryObject(componentWebsite, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "urlChanged",
            target: website,
        });

        compare(spy.count, 0);

        website.changeUrl();

        spy.wait();
        compare(website.url, "unknown");
        compare(spy.count, 1);
    }
}
