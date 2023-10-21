<script>
    import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte'
    import FaExternalLinkAlt from 'svelte-icons/fa/FaExternalLinkAlt.svelte'
    import FaTrash from 'svelte-icons/fa/FaTrash.svelte'
    import FaLocationArrow from 'svelte-icons/fa/FaLocationArrow.svelte'
    import { afterUpdate, onMount } from 'svelte';
    import { setValue } from '../js/settings';
    import browser from "webextension-polyfill";
    import { title } from 'svelte-icons/components/IconBase.svelte';

    export let settings;
    export let page;
    export let entities;
    export let keystore;
    export let address_index;
    export let activeId;
    export let url;

    let wallets = entities.map(e => { return {id: e.public_key, name: e.name, balance: e.incoming_amount - e.outgoing_amount} });

    afterUpdate(() => {
        console.log(activeId);
        wallets = entities.map(e => { return {id: e.public_key, name: e.name, balance: e.incoming_amount - e.outgoing_amount} });
        totalBalance = wallets.map(w => w.balance).reduce((part, a) => part + a, 0);
    });

    let totalBalance = wallets.map(w => w.balance).reduce((part, a) => part + a, 0);
    
    function getIdent(num, fmt) {
        switch (fmt) {
            case "1e0":
                return "";
            case "1e3":
                return "k";
            case "1e6":
                return "Mn";
            case "1e9":
                return "Bn";
            case "auto":
                return getIdent(num, String(fmt_auto(num)))
        }
    }

    function fmt_number(num) {
        switch (settings.num_fmt) {
            case "auto":
                return num/Number(fmt_auto(num));
            case "1e0":
                return num;
            case "1e3":
                return num/1e3;
            case "1e6":
                return num/1e6;
            case "1e9":
                return num/1e9;
            
        }
    }

    function fmt_auto(num) {
        switch (true) {
            case num < 2600:
                return "1e0";
            case num < 26e5:
                return "1e3";
            case num < 26e8:
                return "1e6";
            default:
                return "1e9";
        }
    }

    function short(id) {
        return `${id.slice(0, 4)}...${id.slice(56, 60)}`;
    }

    function delete_wallet(idx) {
        console.log("test", idx);
        entities.splice(idx, 1);
        entities = entities;
        keystore.wallets.splice(idx, 1);

        setValue("keystore", keystore);
    }
</script>

<div id="dashboard">
    <div id="overview">
        <h3>Total Balance:</h3>
        <div>
            {
                `${fmt_number(totalBalance).toLocaleString(undefined, {maximumFractionDigits: 3})} ${getIdent(totalBalance, settings.num_fmt)} QUs`
            }
        </div>
    </div>

    <div id="wallets">
        {#each wallets as wallet, idx}
            <div class="wallet">
                <div class=id-display>
                    <div style="color: gray;">
                        {
                            wallet.name
                        }
                    </div>
                    <div class=id-display-group>
                        <div class=activity-indicator style={wallet.id === activeId ? "background-color: #4CAF50;" : ""} on:keypress={() => {}} on:click={async () => {
                            if (activeId) {
                                activeId = wallet.id;
                                let conn = (await browser.storage.local.get("connectedApp")).connectedApp ?? [];
                                conn.forEach((c,idx) => {
                                    if (c.url == url) {
                                        c.id = activeId;
                                    }
                                });
                                await browser.storage.local.set({"connectedApp": conn});
                            }
                        }}>
                            
                        </div>
                        <div class=clipboard on:keypress={() => {}} on:click={() => {
                            navigator.clipboard.writeText(wallet.id);
                        }}>
                        {
                                short(wallet.id)
                            } 
                        </div>
                        <div style="height: 1em; cursor: pointer;" on:keypress={() => {}} on:click={() => {
                            browser.tabs.create({ url: `https://app.qubic.li/network/explorer/address/${wallet.id}`});
                        }}>
                            <FaExternalLinkAlt/>
                        </div>
                    </div>
                    
                </div>
                <div class=balance-action-display>
                    <div class=balance>
                        <div>
                            {
                                fmt_number(wallet.balance).toLocaleString(undefined, {maximumFractionDigits: 3})
                            }
                            
                        </div>
                        <div>
                            {
                                `${getIdent(wallet.balance, settings.num_fmt)} QUs`
                            }
                        </div>
                    </div>

                    <div class=actions>
                        <div class=action-icon on:keypress={() => {}} on:click={() => { address_index = idx; page = 4; }}>
                            <FaLocationArrow/>
                        </div>
                        <div class=action-icon on:keypress={() => {}} on:click={() => delete_wallet(idx)}>
                            <FaTrash/>
                        </div>
                    </div>
                </div>
            </div>
        {/each}

        <button id=add-button on:click={() => { page = 3; }}>
            <FaPlusCircle/>
        </button>
    </div>
</div>

<style>
    .clipboard {
        cursor: pointer;
    }

    #dashboard {
        height: 100%;
    }

    #overview {
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        font-size: large;

        border-bottom: #27a8db solid 1px;
        margin-bottom: 20px;
    }

    #wallets {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .balance-action-display {
        display: flex;
        height: 100%;
    }

    .balance {
        width: 125px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border-right: #27a8db solid 1px;
        padding-bottom: 5px;
    }

    .action-icon {
        height: 1.5em;
        color: #27a8db;
    }

    .actions {
        width: 125px;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        padding-bottom: 5px;
    }

    .id-display {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        column-gap: 5px;
        padding-bottom: 5px;
        border-bottom: #27a8db solid 1px;
    }

    .id-display-group {
        width: 100%;
        display: flex;
        justify-content: space-evenly;
    }

    .wallet {
        font-size: medium;
        font-weight: bold;
        display: flex;
        flex-direction: column;
        align-items: start;
        border: #27a8db solid 1px;
        border-radius: 10px;
        width: 250px;
        height: 100px;
        margin-bottom: 20px;
        padding-top: 5px;
    }

    #add-button {
		width: 250px;
        height: 40px;
		background-color: #27a8db;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
        
	}
    
    .activity-indicator {
        width: 15px;
        height: 15px;
        border: gray solid 1px;
        border-radius: 15px;
    }

    /*#overview h3 {
        color: #27a8db;
    }*/
</style>