import browser from "webextension-polyfill";


const functions = ["requestAccounts", "sendTransaction", "requestAuthentication"]

let accepted = false;

window.addEventListener("message", async (event) => {
    if (functions.includes(event.data.type)) {
        let activeId = "";
        let connected = false;
        let keystore = (await browser.storage.local.get("keystore")).keystore;
        let url = window.location.origin;

        let conn = (await browser.storage.local.get("connectedApp")).connectedApp ?? [];

        // check if connection is cached
        conn.forEach(c => {
            if (c.url == url) {
                activeId = c.id;
                connected = true;
            }
        });

        if (connected && event.data.type !== "requestAccounts") {
            try {
                await browser.runtime.sendMessage({ type: event.data.type, url: url });
            } catch (e) {
                console.log(e);
            }
            
            // request trigger
            await browser.storage.session.set({"requestFunction": {...event.data, url: url}});

            // response listener
            browser.storage.session.onChanged.addListener(changes => {
                if (changes.responseFunction) {
                    if (changes.responseFunction.newValue.type === "accepted") {
                        window.postMessage({ type: "accepted", value: {...changes.responseFunction.newValue.value} }, "*");
                    } else {
                        window.postMessage({ type: "rejected" }, "*");
                    }
                }
            });
        } else if (!connected && event.data.type === "requestAccounts") {
            try {
                await browser.runtime.sendMessage({ type: event.data.type, url: url });
            } catch (e) {
                console.log(e);
            }
            
            // request trigger
            await browser.storage.session.set({"requestFunction": {...event.data, url: url}});

            // response listener
            browser.storage.session.onChanged.addListener(changes => {
                if (changes.responseFunction) {
                    if (changes.responseFunction.newValue.type === "accepted") {
                        window.postMessage({ type: "accepted", value: {...changes.responseFunction.newValue.value} }, "*");
                    } else {
                        window.postMessage({ type: "rejected" }, "*");
                    }
                }
            });
        } else if (event.data.type === "requestAccounts") {
            let ids = keystore.wallets.map(w => w.id);
            let idx = ids.findIndex(val => val == activeId);
            if (idx !== -1) {
                ids.splice(idx, 1);
                ids.unshift(activeId);
            }

            window.postMessage({ type: "accepted", value: ids}, "*");
        } else {
            window.postMessage({ type: "rejected" }, "*");
        }

        setTimeout(() => {
            if (!accepted) {
                window.postMessage({ type: "rejected" }, "*");
            } else {
                accepted = false;
            }
            
        }, event.data.timeout);
    }
});