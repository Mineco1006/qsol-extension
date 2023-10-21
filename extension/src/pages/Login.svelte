<script>
    import browser from "webextension-polyfill";
	import { check_password, set_password } from "../../../qubic-wasm/pkg/qubic_wasm";
    import { setValue } from "../js/settings";

    export let page;
	export let keystore;

	let password = ""

	async function loginHandler() {
		if (Boolean(keystore.pwd)) {
			if (check_password(keystore, password)) {
				page = 1;
				await browser.storage.session.set({login: { logged_in: true }});
			} else {
				password = ""
				await browser.notifications.create("", {
					type: "basic",
					title: "QSOL Wallet Notification",
					iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
					message: "Invalid password provided!"
				});
			}
		} else {
			keystore = set_password(password);
			console.log(keystore);
			await browser.storage.session.set({login: { logged_in: true }});
			setValue("keystore", keystore);
			page = 1;
		}
	}
</script>

<div id="login">
	<img src="../assets/qbc1024.png" alt=""/>

	<div>
		Enter Password to Continue
	</div>

	<input id="login-input" type="password" bind:value={password}/>

	<button id="unlock-button" on:click={async () => { await loginHandler(); }}>
		{ Boolean(keystore.pwd) ? "UNLOCK" : "SET PASSWORD" }
	</button>
</div>
    

<style>

	img {
		width: 200px;
		-webkit-user-drag: none;
		user-select: none;
		-webkit-user-select: none;
	}

	#login {
		padding-top: 50px;
		display: flex;
		row-gap: 50px;
		flex-direction: column;
		align-items: center;
		font-size: large;
	}

	#login-input {
		width: 250px;
		font-size: large;
		font-family: Verdana;
		border: 1px solid #27a8db;
		border-radius: 10px;
	}

	#unlock-button {
		width: 250px;
		background-color: #27a8db;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
	}
</style>