import browser from "webextension-polyfill";

function to_message(msg) {
    switch (msg.type) {
        case "requestAccounts":
            return `${msg.url} wants to connect to your wallet`;
        case "requestAuthentication":
            return `${msg.url} requests authentication`;
        case "sendTransaction":
            return `${msg.url} requests a transaction signature`;
    }
}

console.log("listener running");

browser.runtime.onMessage.addListener(async (msg) => {
    browser.notifications.create(null, {
        type: "basic",
        title: "QSOL Wallet Notification",
        iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
        message: to_message(msg)
    });

    //chrome-extension://hpadgpbdigehmkmfkgdpccaijocinbim/index.html

    browser.windows.create({
        url: "chrome-extension://hpadgpbdigehmkmfkgdpccaijocinbim/index.html",
        type: "popup",
        height: 660,
        width: 325,
    });
});