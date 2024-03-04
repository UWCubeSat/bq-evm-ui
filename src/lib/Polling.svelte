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

                // getting GCVERF
                returnedByte = await send_serial_command("read bq 27").toString(2);
                VREF_GC_4 = parseInt(returnedByte.charAt(7));
                VREF_OC_4 = parseInt(returnedByte.charAt(6));
                VREF_OC_5 = parseInt(returnedByte.charAt(5));

                returnedByte = await send_serial_command("read bq 16").toString(2);
                VREF_GAIN_CORR = parseInt(returnedByte.substr(0, 4), 2);
                VREF_OFFSET_CORR = parseInt(returnedByte.substr(4, 4), 2);

                GCVREF = (1 + ((VREF_GC_4 << 4)+ VREF_GAIN_CORR) * 0.001) + ((VREF_OC_5 << 5) + (VREF_OC_4 << 4) + VREF_OFFSET_CORR) * 0.001 / vref

                // getting VC_GC_4 and VC_OC_4
				if(i < 2){
					returnedByte = await send_serial_command("read bq 23").toString(2);
					if(i == 0){
						VC_GC_4 = parseInt(returnedByte.charAr(6));
						VC_OC_4 = parseInt(returnedByte.charAr(7));
					} else {
						VC_GC_4 = parseInt(returnedByte.charAr(4));
						VC_OC_4 = parseInt(returnedByte.charAr(5));
					}	
				} else {
					returnedByte = await send_serial_command("read bq 24").toString(2);
					if(i == 2){
						VC_GC_4 = parseInt(returnedByte.charAr(6));
						VC_OC_4 = parseInt(returnedByte.charAr(7));
					} else if(i == 3){
						VC_GC_4 = parseInt(returnedByte.charAr(4));
						VC_OC_4 = parseInt(returnedByte.charAr(5));
					} else if(i == 4){
						VC_GC_4 = parseInt(returnedByte.charAr(2));
						VC_OC_4 = parseInt(returnedByte.charAr(3));
					} else if(i == 5 ){
						VC_GC_4 = parseInt(returnedByte.charAr(0));
						VC_OC_4 = parseInt(returnedByte.charAr(1));
					}
				}

				// getting vc_offset_corr and vc_gain_corr
				returnedByte = await send_serial_command("read bq " + (16 + i)).toString(2);
                VC_GAIN_CORR = parseInt(returnedByte.substr(0, 4), 2);
                VC_OFFSET_CORR = parseInt(returnedByte.substr(4, 4), 2);

				// getting GCVCOUT and OCVCOUT
                GCVCOUT = ((VC_GC_4 << 4) + VC_GAIN_CORR) * 0.001
				OCVCOUT = ((VC_OC_4 << 4) + VC_OFFSET_CORR) * 0.001

                // getting GVCOUT
				returnedByte = await send_serial_command("read bq 4").toString(2);
				REF_SEL = parseInt(returnedByte.charAt(0));
				if(REF_SEL == 0){
					GVCOUT = 0.3;
				} else {
					GVCOUT = 0.6;
				}

                cells[i] = ((cellVoltage * GCVREF + OCVCOUT) / GVCOUT * (1 + GCVCOUT)).toPrecision(4);
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
