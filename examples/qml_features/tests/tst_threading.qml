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

        ThreadingWebsite {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_defaults() {
        const website = createTemporaryObject(componentWebsite, null, {});
        compare(website.url, "https://kdab.com");
        compare(website.title, "KDAB");
    }

    function test_title_refresh() {
        const website = createTemporaryObject(componentWebsite, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "titleChanged",
            target: website,
        });

        compare(spy.count, 0);

        website.fetchTitle();

        spy.wait();
        compare(website.title, "Loading...");
        compare(spy.count, 1);

        spy.wait();
        compare(website.title, "KDAB");
        compare(spy.count, 2);
    }

    function test_url_change() {
        const website = createTemporaryObject(componentWebsite, null, {});
        const titleSpy = createTemporaryObject(componentSpy, null, {
            signalName: "titleChanged",
            target: website,
        });
        const urlSpy = createTemporaryObject(componentSpy, null, {
            signalName: "urlChanged",
            target: website,
        });

        compare(urlSpy.count, 0);
        website.changeUrl();
        compare(website.url, "https://github.com/kdab/cxx-qt");
        compare(urlSpy.count, 1);

        website.fetchTitle();

        titleSpy.wait();
        compare(website.title, "Loading...");
        compare(titleSpy.count, 1);

        titleSpy.wait();
        compare(website.title, "GitHub");
        compare(titleSpy.count, 2);
    }
}
