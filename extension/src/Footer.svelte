<script>
    import { onMount } from "svelte";
    import { get_current_tick } from "../../qubic-wasm/pkg/qubic_wasm";

    export let settings;
    export let current_tick;

    onMount(async () => {
        setInterval(async () => {
            try {
                current_tick = await get_current_tick(settings.rpc);
            } catch {

            }
        }, 5000)
    })
</script>

<div id=footer>
    <div>
        Powered by <a target="_blank" href="https://qubic.solutions/">qubic.solutions</a>
    </div>
    <div>
        Network Tick: {current_tick}
    </div>
</div>

<style>
    #footer {
        position: sticky;
    
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 20px;
        border-top: 1px solid #27a8db;
        padding-left: 10px;
        padding-right: 10px;
    }
</style>