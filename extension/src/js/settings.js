import browser from "webextension-polyfill";

export async function setValue(key, value) {
    await browser.storage.local.set({[key]: value});
}

export async function getValue(key) {
    return await browser.storage.local.get(key);
}