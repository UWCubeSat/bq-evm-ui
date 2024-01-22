<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Badge } from "flowbite-svelte";

    export let portPath: string | null;

    async function close_port() {
        return await invoke("close_serial_port");
    }
</script>

{#if portPath}
    <Badge large dismissable rounded color="green" on:close={async () => {
        portPath = null;
        await close_port();
        }}>{portPath}</Badge>
{:else}
    <Badge large rounded color="red">Not Connected</Badge>
{/if}
