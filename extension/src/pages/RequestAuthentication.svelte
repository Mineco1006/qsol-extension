<script>
    import browser from "webextension-polyfill";
    import { afterUpdate } from "svelte";
    import { sign_authentication_request } from "../../../qubic-wasm/pkg/qubic_wasm";
    import Prompt from "./Prompt.svelte";
    import { short } from "../js/utils";

    export let page;
    export let keystore;
    export let service_name;
    export let activeId;
    let close = true;
    let password = "";
    let activeIdIndex = keystore.wallets.findIndex(w => w.id == activeId);

    afterUpdate(async () => {
        activeIdIndex = keystore.wallets.findIndex(w => w.id == activeId);
        if (Boolean(password)) {
            try {
                if (activeIdIndex != -1) {
                    let auth = sign_authentication_request(keystore, password, service_name, activeIdIndex);
                    console.log(auth);
                    await browser.storage.session.set({"requestFunction": null});
                    await browser.storage.session.set({"responseFunction": { type: "accepted", value: auth}});
                    page = 1;
                }
            } catch(e) {
                console.log(e);
            }
        }
    });
</script>

{#if !close}
    <Prompt bind:password keystore={keystore} bind:close/>
{/if}
<div id=authentication>
    <h2 style="text-align: center">
        Service <div style="color: #27a8db;">{service_name}</div> requests authentication
    </h2>

    <div class=input-box>
        <div>
            Active ID
        </div>
        <input class=input-field disabled placeholder={`${keystore.wallets[activeIdIndex]?.name} | ${short(activeId)}`}/>
    </div>

    <button id=sign on:click={() => { close = false; }}>Sign</button>
</div>

<style>
    #authentication {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        row-gap: 20px;
    }

    #sign {
        width: 250px;
		background-color: #27a8db;
        font-size: large;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
    }

    .input-box {
        display: flex;
        align-items: center;
        justify-content: space-between;
        border: 1px solid #27a8db;
        border-radius: 10px;
        width: 250px;
        height: 50px;
    }

    .input-box :first-child {
        margin-left: 10px;
        width: 80px;
    }

    .input-field {
        height: 50px;
        border-top-right-radius: 10px;
        border-bottom-right-radius: 10px;
        margin-top: 5px;
    }
</style>