<script>
    import browser from "webextension-polyfill";
    import { send_transaction } from "../../../qubic-wasm/pkg/qubic_wasm";
    import { short } from "../js/utils";
    import Prompt from "./Prompt.svelte";
    import { afterUpdate } from "svelte";

    export let keystore;
    export let settings;
    export let current_tick;
    export let activeId;
    export let page;
    export let url;
    export let transaction_request = {
        to: "XOHYYIZLBNOAWDRWRMSGFTOBSEPATZLQYNTRBPHFXDAIOYQTGTNFTDABLLFA",
        amount: 10,
        input_size: 0,
        input_type: 0
    };

    let password = "";
    let timeout = false;
    let close = true;

    let amount;
    let tick_offset = "20";

    afterUpdate(async () => {
        let address_index = keystore.wallets.findIndex(w => w.id == activeId);
        if (Boolean(password) && !timeout) {
            timeout = true;
            try {
                await send_transaction(keystore, password, address_index, settings.rpc, transaction_request.to, BigInt(transaction_request.amount), current_tick + Number(tick_offset), transaction_request.input_type, transaction_request.input_size);
                password = "";

                close = true;
                await browser.storage.session.set({"requestFunction": null});
                await browser.storage.session.set({"responseFunction": { type: "accepted", value: {from: activeId, to: transaction_request.to, amount: transaction_request.amount, tick: current_tick + Number(tick_offset), input_type: transaction_request.input_type, input_size: transaction_request.input_size}}});

                browser.notifications.create("", {
                    type: "basic",
                    title: "QSOL Wallet Notification",
                    iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
                    message: `Requested ${url} transaction ${activeId} -> ${transaction_request.to} | Amount: ${transaction_request.amount} QUs\nwas submitted successfully for tick ${current_tick + Number(tick_offset)}`
                });
                page = 1;
            } catch(e) {
                password = "";
                close = true;

                await browser.notifications.create("", {
                    type: "basic",
                    title: "QSOL Wallet Notification",
                    iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
                    message: `RPC error while submitting transaction ${e}`
                });
                page = 1;
            }
            
        }
    });
</script>

{#if !close}
    <Prompt bind:password keystore={keystore} bind:close/>
{/if}
<div id=send-transaction>
    <div class=input-box>
        <div>
            Requested By:
        </div>
        <input class=input-field disabled placeholder={url}/>
    </div>
    <div class=input-box>
        <div>
            From:
        </div>
        <input class=input-field disabled placeholder={short(activeId)}/>
    </div>
    <div class=input-box>
        <div>
            To:
        </div>
        <input class="input-field id-input" type="text" disabled placeholder={short(transaction_request.to)}/>
    </div>
    <div class=input-box>
        <div>
            Amount:
        </div>
        <input class=input-field type="number" disabled placeholder={transaction_request.amount}/>
    </div>
    <div class=input-box>
        <div>
            Input Type:
        </div>
        <input class=input-field type="number" disabled placeholder={transaction_request.input_type}/>
    </div>
    <div class=input-box>
        <div>
            Input Size:
        </div>
        <input class=input-field type="number" disabled placeholder={transaction_request.input_size}/>
    </div>
    <div class=input-box>
        <div>
            Tick Offset:
        </div>
        <select class=input-field bind:value={tick_offset}>
            <option value="10">10</option>
            <option value="20">20</option>
            <option value="30">30</option>
            <option value="40">40</option>
            <option value="50">50</option>
        </select>
    </div>

    <button id=submit-button on:click={() => { close = false; }}>
        Sign Transaction
    </button>
</div>

<style>
    #send-transaction {
        display: flex;
        align-items: center;
        flex-direction: column;
        row-gap: 10px;
        padding-top: 10px;
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

    .id-input {
        font-size: small;
    }

    #submit-button {
		width: 250px;
		background-color: #27a8db;
        font-size: large;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
	}
</style>