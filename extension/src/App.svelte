<script>
    import { onMount } from "svelte";
	import Dashboard from "./pages/Dashboard.svelte";
	import Login from "./pages/Login.svelte";
    import Navbar from "./Navbar.svelte";
    import Settings from "./pages/Settings.svelte";
	import browser from "webextension-polyfill";
	import { getValue } from "./js/settings";
	import init, { get_entities, get_current_tick } from "../../qubic-wasm/pkg/qubic_wasm";
    import Footer from "./Footer.svelte";
    import AddWallet from "./pages/AddWallet.svelte";
    import SendTransaction from "./pages/SendTransaction.svelte";
    import RequestAccounts from "./pages/RequestAccounts.svelte";
    import RequestAuthentication from "./pages/RequestAuthentication.svelte";
    import RequestTransaction from "./pages/RequestTransaction.svelte";

	let keystore = {
		pwd: "",
		salt: [0, 0, 0, 0],
		wallets: []
	}

	let settings = {
		rpc: "https://rpc.qubic.solutions",
		num_fmt: "1e0"
	}

	let activeId = "";
	let address_index = 0;

	let entities = [];
	let page = 0;
	let current_tick = 0;
	let url = "http://127.0.0.1/";
	let transaction_request;

	let service_name;

	const Functions = {
		RequestAccounts: "requestAccounts",
		SendTransaction: "sendTransaction",
		RequestAuthentication: "requestAuthentication"
	}

	onMount(async () => {
		await init();
		await browser.storage.session.setAccessLevel({ accessLevel: 'TRUSTED_AND_UNTRUSTED_CONTEXTS' });

		entities = (await browser.storage.local.get("cachedEntities")).cachedEntities ?? [];

		let login = (await browser.storage.session.get("login"));

		let isettings = (await getValue("settings")).settings ?? settings;
        console.log(settings);
        settings.rpc = isettings.rpc ?? settings.rpc;
        settings.num_fmt = isettings.num_fmt ?? settings.num_fmt;

		let requestFunction = (await browser.storage.session.get("requestFunction")).requestFunction;

		let tab = await browser.tabs.query({ active: true, currentWindow: true });
		url = (new URL(tab[0].url)).origin;

		let conn = (await browser.storage.local.get("connectedApp")).connectedApp ?? [];

		if (login.login?.logged_in) {
			page = 1;
		}

		if (requestFunction) {
			switch (requestFunction.type) {
				case Functions.RequestAccounts:
					url = requestFunction.url;
					page = 5;
					break;
				case Functions.RequestAuthentication:
					service_name = requestFunction.service_name;
					url = requestFunction.url;
					page = 6;
					break;
				case Functions.SendTransaction:
					transaction_request = requestFunction;
					url = requestFunction.url;
					page = 7;
					break;
			}
		}

		keystore = (await getValue("keystore")).keystore ?? keystore;

		let ids = keystore.wallets.map(w => w.id);

		// check for previous connection
        conn.forEach(c => {
            if (c.url == url) {
				if (ids.includes(c.id)) {
					activeId = c.id;
				} else {
					activeId = ids[0] ?? "";
				}
            }
        });

		try {
            current_tick = await get_current_tick(settings.rpc);
        } catch(e) {
			await browser.notifications.create("", {
				type: "basic",
				title: "QSOL Wallet Notification",
				iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
				message: `RPC request errored!\nCurrent URL: ${settings.rpc}\nPlease enter a valid RPC in the settings page!\n ${e}`
			});
        }

		console.log(keystore);

		try {
			let nentities = (await get_entities(keystore, settings.rpc));

			if (nentities?.length === keystore.wallets.length) {
				entities = nentities;
				keystore.wallets.forEach((w, idx) => {
					entities[idx].name = w.name;
				});

				await browser.storage.local.set({"cachedEntities": entities});
			}
		} catch(e) {
			console.log(e);
		}
	});
</script>

<main>
	{#if page !== 0}
		<Navbar bind:page/>
	{/if}
	{#if page === 0}
		<Login bind:page bind:keystore/>
	{:else if page === 1}
		<Dashboard settings={settings} bind:entities bind:page bind:keystore bind:address_index bind:url bind:activeId/>
	{:else if page === 2}
		<Settings bind:settings/>
	{:else if page === 3}
		<AddWallet bind:keystore bind:page bind:entities bind:settings/>
	{:else if page === 4}
		<SendTransaction bind:keystore bind:page bind:address_index bind:settings bind:current_tick/>
	{:else if page === 5}
		<RequestAccounts bind:keystore bind:url bind:page/>
	{:else if page === 6}
		<RequestAuthentication bind:keystore bind:page bind:service_name bind:activeId/>
	{:else if page === 7}
		<RequestTransaction bind:keystore bind:page bind:settings bind:transaction_request bind:current_tick bind:activeId bind:url/>
	{/if}

	<Footer settings={settings} bind:current_tick/>
</main>

<style>
	/* window props */
	main {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		width: 320px;
		height: 600px;
		border-radius: 5px;
		padding: 5px, 5px, 5px, 5px;
	}
</style>