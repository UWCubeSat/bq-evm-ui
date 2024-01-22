<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Button, ButtonGroup, Input } from "flowbite-svelte";
    import { afterUpdate } from "svelte";

    export let port: string | null;

    let logElement:Element;
    let logIndex = 0;
    let log: string[] = [];
    let data: string = "";
    
    const scrollToBottom = async (node:Element) => {
        node.scroll({ top: node.scrollHeight, behavior: 'smooth' });
    };

    afterUpdate(() => {
		if(log) scrollToBottom(logElement);
    });
	
    $: {
        if(log && logElement) {
            scrollToBottom(logElement);
        }
    }

    async function read_from_serial_port() {
        let response:string = await invoke("read_from_serial_port");
        return response;
    }

    async function write_serial_port() {
        await invoke("write_to_serial_port", {data: data + "\n"});
        let date:Date = new Date();
        log[logIndex++] = date.toISOString() + " (UI): " +  data;
        let response = await read_from_serial_port();
        date = new Date();
        log[logIndex++] = date.toISOString() + " (EVM): " + response.substring(0, response.lastIndexOf('\n'));
        data = "";
    }

    afterUpdate(() => {
        if(log) scrollToBottom(logElement);
    })

</script>

<div bind:this={logElement} class="h-[250px] overflow-auto">
	{#each log as l}
        <p class="text-sm">{l}</p>
    {/each}
</div>

<form class="row" on:submit|preventDefault={write_serial_port}>
    <div class="grid grid-cols-2 flex">
        <div class="flex-1">
            <Input bind:value={data} disabled={port == null}></Input>
        </div>
        <div class="text-right">
            <ButtonGroup>
            <Button color="blue" on:click={() => {log = []; logIndex = 0;}} disabled={port == null}>Clear</Button>
            <Button color="green" type="submit" disabled={port == null}>Send</Button>
            </ButtonGroup>
        </div>
    </div>
</form>
