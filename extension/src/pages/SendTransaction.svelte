<script>
    import browser from "webextension-polyfill";
    import { send_transaction } from "../../../qubic-wasm/pkg/qubic_wasm";
    import { short } from "../js/utils";
    import Prompt from "./Prompt.svelte";
    import { afterUpdate } from "svelte";

    export let keystore;
    export let settings;
    export let address_index;
    export let current_tick;
    export let page;

    console.log(keystore, address_index);

    let password = "";
    let close = true;
    let timeout = false;

    let to = "";
    let amount;
    let tick_offset = "20";

    afterUpdate(async () => {
        if (Boolean(password) && !timeout) {
            timeout = true;
            try {
                await send_transaction(keystore, password, address_index, settings.rpc, to, BigInt(amount), current_tick + Number(tick_offset), 0, 0)
                password = "";
                close = true;

                await browser.notifications.create("", {
                    type: "basic",
                    title: "QSOL Wallet Notification",
                    iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
                    message: `Transaction ${short(keystore.wallets[address_index].id)} -> ${short(to)} | Amount: ${BigInt(amount)} QUs\nwas submitted successfully for tick ${current_tick + Number(tick_offset)}`
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
            From:
        </div>
        <input class=input-field disabled placeholder={`${keystore.wallets[address_index].name} | ${short(keystore.wallets[address_index].id)}`}/>
    </div>
    <div class=input-box>
        <div>
            To:
        </div>
        <input list=known-addresses class="input-field id-input" type="text" bind:value={to}/>
        <datalist id=known-addresses on:select={e => { to = e.target.value; }}>
            {#each keystore.wallets as wallet, idx}
                {#if idx !== address_index}
                    <option value={wallet.id}>{wallet.name}</option>
                {/if}
            {/each}
        </datalist>
    </div>
    <div class=input-box>
        <div>
            Amount:
        </div>
        <input class=input-field type="number" bind:value={amount}/>
    </div>
    <div class=input-box>
        <div>
            Tick Offset:
        </div>
        <select class=input-field bind:value={tick_offset}>
            <option value="10">10</option>
            <option selected value="20">20</option>
            <option value="30">30</option>
            <option value="40">40</option>
            <option value="50">50</option>
        </select>
    </div>

    <button id=submit-button on:click={() => { close = false; }}>
        Submit Transaction
    </button>
</div>

<style>
    #send-transaction {
        display: flex;
        align-items: center;
        flex-direction: column;
        row-gap: 20px;
        padding-top: 20px;
    }

    .input-box {
        display: flex;
        align-items: center;
        justify-content: space-between;
        border: 1px solid #27a8db;
        border-radius: 10px;
        width: 250px;
        margin-bottom: 20px;
        height: 50px;
    }

    .input-box div {
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