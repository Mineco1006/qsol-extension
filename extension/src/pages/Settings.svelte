<script>
    import { onMount, afterUpdate } from "svelte";
    import { setValue } from "../js/settings";

    export let settings;

    async function save() {
        await setValue("settings", settings);
    }
</script>

<div id=settings>

    <div class=input-box>
        <div>
            RPC:
        </div>
        <input class=input-field style="width: 200px;" bind:value={settings.rpc}/>
    </div>

    <div class=input-box>
        <div>
            FMT:
        </div>
        <select class=input-field bind:value={settings.num_fmt}>
            <option value="1e0">None</option>
            <option value="auto">Auto</option>
            <option value="1e3">k</option>
            <option value="1e6">Mn</option>
            <option value="1e9">Bn</option>
        </select>
    </div>

    <button id=save on:click={async () => {await save()}}>SAVE</button>
</div>

<style>
    input {
        width: 150px;
        background-color: transparent;
        border: none;
    }

    #save {
        width: 250px;
		background-color: #27a8db;
		border: none;
        border-radius: 10px;
        color: whitesmoke;
    }

    #save:active {
        background-color: #277bdb;
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

    .input-box :first-child {
        margin-left: 10px;
        width: 80px;
    }

    .input-field {
        height: 50px;
        border-top-right-radius: 10px;
        border-bottom-right-radius: 10px;
        margin-top: 8px;
    }

    #settings {
        height: 100%;
        margin-top: 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        font-size: large;
    }
</style>