<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { Button } from "flowbite-svelte";
    
    let portStatus: boolean = false;

    async function open_port() {
        portStatus = await invoke("open_serial_port", {portName: "/dev/ttyACM0", baudRate: 9600});
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={open_port}>
        <Button color="green" type="submit">Open Port</Button>
    </form>

    <p>{portStatus}</p>
</div>
