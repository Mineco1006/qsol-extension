<script>
    import browser, { storage } from "webextension-polyfill";
    import { onMount, afterUpdate } from 'svelte';
    import FaArrowLeft from 'svelte-icons/fa/FaArrowLeft.svelte'
    import IoIosSettings from 'svelte-icons/io/IoIosSettings.svelte'
    export let page;
    let pageName = getPageName();

    afterUpdate(() => {
        pageName = getPageName();
    })

    function getPageName() {
		switch (page) {
			case 1:
				return "Dashboard";
			case 2:
				return "Settings";
            case 3:
				return "Add Wallet";
            case 4:
                return "Send Transaction";
            case 5:
                return "Request Accounts";
            case 6:
                return "Request Authentication";
            case 7:
                return "Request Transaction Signature";
			default:
				return "Dashboard";
		}
	}
</script>

<div id="navbar">
    {#if page == 1}
        <img src="../assets/qbc1024.png" alt=""/>
        
        <div>
            { pageName }
        </div>

        <div class="icon" on:click={() => { page = 2 }} on:keypress={() => {}}>
            <IoIosSettings />
        </div>
    {:else}
        <div class="icon" on:click={async () => {
            await browser.storage.session.set({"requestFunction": null});
            await browser.storage.session.set({"responseFunction": { type: "rejected" }});
            page = 1;
            }} on:keypress={() => {}}>
            <FaArrowLeft/>
        </div>
        <div>
            { pageName }
        </div>

        <div style="width: 50px;">
        </div>
    {/if}
</div>

<style>
    #navbar {
        background-color: rgb(225, 225, 225);
        width: 320px;
        height: 50px;
        display: flex;
        padding: 2px;
        justify-content: space-between;

        border-bottom: 1px solid #27a8db;
    }

    .icon {
        color: #57595D;
        width: 50px;
    }
</style>