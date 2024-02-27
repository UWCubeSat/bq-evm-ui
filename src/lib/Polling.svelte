<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Label } from "flowbite-svelte";
    import { onDestroy, onMount } from "svelte";

    let timer: any;
    let vref: string = "";
    let vref_num: number = 0;
    let vtemp: string = "";
    let vtemp_adc: string = "";

    let cells_adc: string[] = [];
    let cells: string[] = [];

    async function read_from_serial_port() {
        let response:string = await invoke("read_from_serial_port");
        return response;
    }

    async function send_serial_command(data: string) {
        await invoke("write_to_serial_port", {data: data + "\n"});
        let response = await read_from_serial_port();
        return response;
    }

    onMount(async () => {
        // Setup the BQ for Polling.
        await send_serial_command("write bq 4 1");      // VREF to 3.0;
        await send_serial_command("write bq 5 31");     // POWER_CTL Enable;

        timer = setInterval(async () => {
            // VREF
            await send_serial_command("write bq 1 32");
            vref_num = parseInt(await send_serial_command("read ard 5"))/1024 * 3.3 * 2 * 1000
            vref = vref_num.toPrecision(4);

            // Vtemp
            await send_serial_command("write bq 1 22");
            vtemp_adc = await send_serial_command("read ard 5");
            vtemp = ((1317 - (parseInt(vtemp_adc)/1024 * 3.3 * 1000))/4.018).toPrecision(4);

            // Cells.
            for(let i = 0; i < 6; i++) {
                await send_serial_command("write bq 1 " + (i + 16));
                cells_adc[i] = await send_serial_command("read ard 5");
                let cellVoltage = parseInt(cells_adc[i])/1024 * 3.3;

                returnedByte = await send_serial_command("read bq 27").toString(2);
                VREF_GC_4 = parseInt(returnedByte.charAt(7));
                VREF_OC_5 = parseInt(returnedByte.charAt(5));
                
                VREF_GAIN_CORR =
                VREF_OC_4 = 
                VREF_OFFSET_CORR = 

                GCVREF = (1 + ((VREF_GC_4 << 4)+ VREF_GAIN_CORR) * 0.001) + ((VREF_OC_5 << 5) + (VREF_OC_4 << 4) + VREF_OFFSET_CORR) * 0.001 / vref

                // We don't use all of the Correction Data.
                // This gets us within ~5 mV.
                cells[i] = (cellVoltage * 1000 / 0.6).toPrecision(4);
            }
        }, 500);
    })

    onDestroy(() => {
        clearInterval(timer);
    })
</script>

<div>
    <div class="grid grid-cols-3 text-center gap-y-3">
        <div>
            <Label class="italic text-xs">V_temp (ADC)</Label>
            <p class="font-bold">{vtemp_adc}<p>
        </div>
        <div>
            <Label class="italic text-xs">V_temp (C)</Label>
            <p class="font-bold">{vtemp}<p>
        </div>
        <div>
            <Label class="italic text-xs">V_ref (mV)</Label>
            <p class="font-bold">{vref}<p>
        </div>
        {#each cells as ca, i}
        <div>
            <Label class="italic text-xs">Cell {i+1} (mV)</Label>
            <p class="font-bold">{ca}<p>
        </div>
        {/each}
    </div>
</div>
