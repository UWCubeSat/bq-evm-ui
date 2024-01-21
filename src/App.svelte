<script lang="ts">
    import { Tabs, TabItem } from 'flowbite-svelte';
    import PrintPorts from './lib/PrintPorts.svelte';
    import OpenPort from './lib/OpenPort.svelte';
    import ReadPort from './lib/ReadPort.svelte';

    // We will have two screens.
    // One for the MSP430
    // One for the BQ
    
    // Both of these will allow for you to send a Serial Command
    // Both will let you read a response
</script>

<main>
    <h1 class="text-center my-[2%]">BQ-EVM Eval Tool</h1>

    <Tabs style="full">
      <TabItem open title="MSP430">
        <p class="text-sm text-gray-500 dark:text-gray-400">
          <b>Profile:</b>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        </p>
      </TabItem>
      <TabItem title="BQ76925">
        <p class="text-sm text-gray-500 dark:text-gray-400">
          <b>Settings:</b>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        </p>
      </TabItem>
      <TabItem title="Utilities">
        <div class="text-center">
            <PrintPorts />
            <OpenPort />
            <ReadPort />
        </div>
      </TabItem>
    </Tabs>

</main>
