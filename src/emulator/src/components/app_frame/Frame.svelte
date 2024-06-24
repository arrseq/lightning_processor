<script lang="ts">
    import TitleBar from "./frame/TitleBar.svelte";
    import Body from "./frame/Body.svelte";
    import type TabInfo from "./frame/tab_info";

    export let title = "...";
    export let primary = false;
    export let tabs: TabInfo[] = [];
    export let row_mode = false;
</script>

<div class="root">
    <TitleBar primary={primary} title={title} tabs={tabs} />
    
    <div class="main" class:row_mode={row_mode}>
        <Body>
            <slot />
        </Body>
    </div>
</div>

<style lang="scss">
    .root {
        display: flex;
        flex-direction: column;
        height: 100%;
        max-width: 100%;
        flex: 1;

        .main {
            height: 100%;
            display: flex;
            flex-direction: column;

            &.row_mode {
                flex-direction: row;
            }   
        }
    }
</style>