<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Button, ButtonGroup, Input, Textarea } from "flowbite-svelte";

    export let port: string | null;


    let logIndex = 0;
    let log: string[] = new Array(10);
    let logString: string = "";
    let data: string = "";

    $: {
        logString = log.join("\n");
    }

    async function read_from_serial_port() {
        let response:string = await invoke("read_from_serial_port");
        return response;
    }

    async function write_serial_port() {
        await invoke("write_to_serial_port", {data: data + "\n"});
        let response = await read_from_serial_port();
        let date:Date = new Date();
        log[logIndex] = date.toISOString() + " (UI): " +  data;
        logIndex = (logIndex + 1) % 10;
        log[logIndex] = date.toISOString() + " (EVM): " + response.substring(0, response.lastIndexOf('\n'));
        logIndex = (logIndex + 1) % 10;
        data = "";
    }

</script>

<Textarea bind:value={logString} unWrappedClass="resize-none h-[45vh]" disabled />

<form class="row" on:submit|preventDefault={write_serial_port}>
    <div class="grid grid-cols-2 flex">
        <div class="flex-1">
            <Input bind:value={data} disabled={port == null}></Input>
        </div>
        <div class="text-right">
            <ButtonGroup>
            <Button color="blue" on:click={() => {log = new Array(10);}} disabled={port == null}>Clear</Button>
            <Button color="green" type="submit" disabled={port == null}>Send</Button>
            </ButtonGroup>
        </div>
    </div>
</form>
