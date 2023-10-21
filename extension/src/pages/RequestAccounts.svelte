<script>
    import browser from "webextension-polyfill";
    export let keystore;
    export let url;
    export let page;

    export let enabled = 0;

    function short(id) {
        return `${id.slice(0, 4)}...${id.slice(56, 60)}`;
    }

    async function connect() {
        let ids = keystore.wallets.map(w => { return w.id });
        ids.splice(enabled, 1);
        ids.unshift(keystore.wallets[enabled].id,);
        await browser.storage.session.set({"responseFunction": {type: "accepted", value: ids}});
        await browser.storage.session.set({"requestFunction": null});
        let conn = (await browser.storage.local.get("connectedApp")).connectedApp ?? [];

        conn.push({url: url, id: keystore.wallets[enabled].id});
        await browser.storage.local.set({"connectedApp": conn});

        page = 1
    }
</script>

<div id=request-accounts>
    <div id=message>
        {url} wants to connect to your wallet
    </div>
    {#each keystore.wallets as wallet, idx}
        <div class=wallet on:keypress={() => {}} on:click={() => { enabled = idx }}>
            <div id=indicator style={enabled == idx ? "background-color: #4CAF50;" : "background-color: transparent;"}>

            </div>
            <div style="display: flex; flex-direction: column; width: 200px;">
                <div style="color: gray; width: 100%">
                    {
                        wallet.name
                    }
                </div>
                <div>
                    {
                        short(wallet.id)
                    } 
                </div>
            </div>
        </div>
    {/each}

    <button id=connect on:click={connect}>Connect</button>
</div>

<style>
    #request-accounts {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    #message {
        max-width: 250px;
        overflow-x: hidden;
        font-size: x-large;
        font-weight: bold;
        text-align: center;
        margin-bottom: 50px;
    }

    .wallet {
        font-size: medium;
        font-weight: bold;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        border: #27a8db solid 1px;
        border-radius: 10px;
        width: 250px;
        height: 50px;
        margin-bottom: 20px;
    }

    .wallet #indicator {
        width: 20px;
        height: 20px;
        border: gray solid 1px;
        border-radius: 20px;
    }

    #connect {
        width: 250px;
		background-color: #27a8db;
        font-size: large;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
    }
</style>