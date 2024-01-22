<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { Alert, Badge, Button, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
    import { onMount } from "svelte";
    import StatusCode from "./StatusCode";
    
    export let port: string | null = null;
    let portError: string = "";

    let serialPortList:string[] = [];
    async function print_ports() {
        serialPortList = await invoke("print_serial_ports");
    }


    async function open_port(port_name: string) {
        // Only 9600 Baud for now.
        let portResponse: StatusCode = await invoke("open_serial_port", {portName: port_name, baudRate: 9600});

        switch (portResponse) {
            case StatusCode.SUCCESS: {
                port = port_name;
                break;
            }

            case StatusCode.NO_DEVICE: {
                portError = port_name + ": Device is not available"; 
                break;
            }

            case StatusCode.UNKNOWN: {
                portError = port_name + ": Error Opening Port (In use by another process?)"; 
                break;
            }

            default: {
                portError = port_name + ": " + portResponse;
                break;
            }
        }
    }

    // Fetches Ports on Mount.
    onMount(async () => {
        await print_ports();
    })
</script>

<div>
    {#if portError.length > 0}
        <Alert dismissable color="red" class="mb-[2%]" on:close={() => {portError = ""}}>{portError}</Alert>
    {/if}

    <form class="row" on:submit|preventDefault={print_ports}>
        <Button color="green" type="submit">Refresh Ports</Button>
    </form>

    {#if serialPortList.length > 0}
    <div class="mt-[2%] m-auto w-[65%]"> 
        <Table striped shadow>
            <TableHead>
                <TableHeadCell class="text-center">Port</TableHeadCell>
                <TableHeadCell class="text-center">Action</TableHeadCell>
            </TableHead>

            <TableBody>
                {#each serialPortList as p}
                    <TableBodyRow>
                        <TableBodyCell tdClass="px-2 py-4 whitespace-nowrap font-medium text-center">
                            <Badge rounded large color="blue">{p}</Badge>
                        </TableBodyCell>
                        <TableBodyCell tdClass="px-2 py-4 whitespace-nowrap font-medium text-center">
                            <Button disabled={p == port} color="green" type="submit" class="text-right" on:click={async () => {
                                await open_port(p);
                            }}>Open</Button>
                        </TableBodyCell>
                    </TableBodyRow>
                {/each}

            </TableBody>
        </Table>
    </div>
    {/if}
</div>
