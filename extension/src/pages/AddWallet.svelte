<script>
    import FaRandom from 'svelte-icons/fa/FaRandom.svelte';
    import FaCopy from 'svelte-icons/fa/FaCopy.svelte';
    import Prompt from "./Prompt.svelte";
    import { add_wallet, get_id, get_random_seed, get_entities } from '../../../qubic-wasm/pkg/qubic_wasm';
    import { afterUpdate } from 'svelte';
    import { setValue } from '../js/settings';

    let password = "";
    let close = true;
    export let keystore;
    export let page;
    export let entities;
    export let settings;
    let seed = "";
    let name = `Wallet-${keystore.wallets.length + 1}`;

    let placeholder = "Seed must be 55 lowercase characters"
    let valid = false;

    afterUpdate(async () => {
        if (seed.length == 55) {
            try {
                placeholder = get_id(seed);
                valid = true;
            } catch {

            }
        } else {
            placeholder = "Seed must be 55 lowercase characters"
            valid = false;
        }

        if (Boolean(password) && seed.length == 55) {
            keystore = add_wallet(keystore, password, name, seed)
            setValue("keystore", keystore);

            try {
                entities = (await get_entities(keystore, settings.rpc)) ?? [];
                keystore.wallets.forEach((w, idx) => {
                    entities[idx].name = w.name;
                });
            } catch(e) {
                console.log(e);
                entities = keystore.wallets.map(w => {
                    return {
                        id: w.id,
                        name: w.name,
                        incoming_amount: 0,
                        outgoing_amount: 0
                    }
                });
            }

            page = 1;
        }
    });

    function randomize() {
        seed = get_random_seed();
    }

    function copy() {
        if (valid) {
            navigator.clipboard.writeText(seed);
        }
    }

    function short(id) {
        return `${id.slice(0, 4)}...${id.slice(56, 60)}`;
    }
</script>

{#if !close}
    <Prompt bind:password keystore={keystore} bind:close/>
{/if}
<div id=add-wallet>
    <input id=id-output autocomplete="off" placeholder="Wallet Name" bind:value={name}/>

    <input id=id-output autocomplete="off" placeholder="Seed" bind:value={seed}/>

    <div style="display: flex; width: 250px; justify-content: space-evenly;">
        <button class=randomizer on:click={randomize}>
            <FaRandom/>
        </button>

        <button class=randomizer on:click={copy}>
            <FaCopy/>
        </button>
    </div>
    

    <input disabled id=id-output style="text-align: center;" placeholder={valid ? short(placeholder) : placeholder}/>

    <button type="submit" id=unlock-button on:click={() => { close = false }}>Add Wallet</button>
</div>

<style>
    #add-wallet {
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
        font-size: large;
        column-gap: 10px;
    }

    #id-output {
        font-size: small;
		width: 300px;
        height: 40px;
		font-family: Verdana;
		border: 1px solid #27a8db;
		border-radius: 10px;
	}

	#unlock-button {
		width: 250px;
		background-color: #27a8db;
        font-size: large;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
	}

    .randomizer {
        display: flex;
        justify-content: center;
        align-items: center;
		background-color: #27a8db;
		border: none;
		border-radius: 10px;
		color: whitesmoke;
        height: 2.5em;
    }

</style>