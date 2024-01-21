<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { Button } from "flowbite-svelte";
    
    let serialPortList:string[] = [];

    async function print_ports() {
        let returnMsg: string[] = await invoke("print_ports");
        serialPortList = returnMsg;
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={print_ports}>
        <Button color="green" type="submit">Print Ports</Button>
    </form>

    {#each serialPortList as port}
        <p>{port}</p>
    {/each}
</div>
