<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { Button } from "flowbite-svelte";
    
    let portData: string = "EMPTY";

    async function read_port() {
        portData = await invoke("read_from_serial_port");
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={read_port}>
        <Button color="green" type="submit">Read Port</Button>
    </form>

    <p>{portData}</p>
</div>
