<script lang="ts">
    import TitleBar from "./frame/TitleBar.svelte";
    import Body from "./frame/Body.svelte";
    import type TabInfo from "./frame/tab_info";
    import DragBar from "./DragBar.svelte";

    export let title = "...";
    export let primary = false;
    export let tabs: TabInfo[] = [];
    export let row_mode = false;

    export let focus_height = 100;

    const SLOTS = $$slots;

    if (!SLOTS.default && !SLOTS.focus) {
        throw Error("Atleast one slot must be provided");
    }
</script>

<div class="root">
    <TitleBar primary={primary} title={title} tabs={tabs} />
    
    <div class="main" class:row_mode={row_mode}>
        {#if SLOTS.default}
            <Body>
                <slot />
            </Body>
        {/if}

        {#if SLOTS.default && SLOTS.focus}
            <DragBar on:from_v={(offset) => {
                if (row_mode) return;
                focus_height += offset.detail;
            }} />
        {/if}

        {#if SLOTS.focus}
            <div class="focus" style={`height: ${focus_height}px;`}>
                <slot name="focus" />
            </div>
        {/if}
    </div>
</div>

<style lang="scss">
    @import "../../conf/surface.scss";

    .root {
        display: flex;
        flex-direction: column;
        height: 100%;
        max-width: 100%;
        flex: 1;
        overflow: hidden;

        .main {
            flex: 1;
            display: flex;
            flex-direction: column;

            .focus {
                background: $surface__root;
                display: flex;
                align-items: center;
                justify-content: center;
                flex-direction: column;
                flex: 1;
            }

            &.row_mode {
                flex-direction: row;
            }   
        }
    }
</style>