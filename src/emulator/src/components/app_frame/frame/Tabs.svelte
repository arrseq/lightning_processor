<script lang="ts">
    import Tab from "./Tab.svelte";
    import type TabInfo from "./tab_info";

    export let tabs: TabInfo[] = [];
</script>

<div class="root">
    {#each tabs as tab}
        <Tab disabled={tab.disabled} description={tab.description} label={tab.label} />
    {/each}
</div>

<style lang="scss">
    .root {
        display: flex;
    }
</style>