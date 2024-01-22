<script lang="ts">
    import { Tabs, TabItem } from 'flowbite-svelte';
    import Connect from './lib/Connect.svelte';
    import PortHeader from './lib/PortHeader.svelte';
    import Terminal from './lib/Terminal.svelte';

    let portPath: string | null = null;
</script>

<main>
        <div class="grid grid-cols-2 text-center align-item-center my-[2.5%]">
            <div>
                <h1 class="text-center text-xl font-bold">BQ-EVM-UI</h1>
            </div>
            <div>
                <PortHeader bind:portPath />
            </div>
        </div>

    <div class="w-[90%] m-auto">
        <Tabs style="full" defaultClass="flex rounded-lg divide-x rtl:divide-x-reverse divide-gray-400 shadow dark:divide-gray-700">
            <TabItem open title="Connect" class="w-full">
                <div class="text-center">
                    <Connect bind:port={portPath} />
                </div>
            </TabItem>
            <TabItem title="Terminal" class="w-full">
                <Terminal port={portPath} />
            </TabItem>
        </Tabs>
    </div>
</main>
