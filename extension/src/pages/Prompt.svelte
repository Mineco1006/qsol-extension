<script>
    import browser from "webextension-polyfill";
    import { check_password } from "../../../qubic-wasm/pkg/qubic_wasm";
    export let close;
    export let keystore;
    export let password = ""
    let ipassword = ""

    async function loginHandler() {
        if (check_password(keystore, ipassword)) {
            password = ipassword;
            close = true;
        } else {
            ipassword = ""
            await browser.notifications.create("", {
				type: "basic",
				title: "QSOL Wallet Notification",
				iconUrl: browser.runtime.getURL("assets/qbc1024.png"),
				message: "Invalid password provided!"
			});
        }
	}
</script>

<div id=prompt>
    <div id=prompt-limiter>
        <div id=prompt-inner>
            <div>
                {
                    `Enter Password to Continue`
                }
            </div>
            <input id="pwd-input" type="password" bind:value={ipassword}/>

            <div id=action-tab>
                <button on:click={() => { close = true; }} style="background-color: #f44336;">Cancel</button>
                <button style="background-color: #4CAF50;" on:click={loginHandler}>Submit</button>
            </div>
        </div>
    </div>
</div>

<style>
    #prompt {
        display: block;
        position: absolute;
        background-color: rgba(10, 10, 10, 0.2);
        width: 320px;
		height: 600px;
    }

    #prompt-limiter {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 320px;
		height: 600px;
    }

    #prompt-inner {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        background-color: whitesmoke;
        border-radius: 20px;
        height: 200px;
        width: 250px;
        padding-top: 30px;
        font-size: large;
    }

    #pwd-input {
		width: 200px;
		font-size: large;
		font-family: Verdana;
		border: 1px solid #27a8db;
		border-radius: 10px;
	}

    #action-tab {
        display: flex;
        justify-content: space-evenly;
        width: 250px;
    }

    #action-tab button {
        width: 80px;
        border: none;
        border-radius: 10px;
        color: whitesmoke
    }
</style>