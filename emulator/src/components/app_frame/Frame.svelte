<script lang="ts">
    import TitleBar from "./frame/TitleBar.svelte";
    import Body from "./frame/Body.svelte";
    import type TabInfo from "./frame/tab_info";
    import DragBar from "./DragBar.svelte";
    import Divider from "./Divider.svelte";

    export let title = "...";
    export let primary = false;
    export let tabs: TabInfo[] = [];
    export let row_mode = false;

    const SLOTS = $$slots;

    if (!SLOTS.default && !SLOTS.focus) {
        throw Error("Atleast one slot must be provided");
    }
</script>

<div class="root">
    <TitleBar primary={primary} title={title} tabs={tabs} />
    
    <div class="main" class:row_mode={row_mode}>
        {#if SLOTS.default && !SLOTS.focus}
            <Body>
                <slot />
            </Body>
        {/if}

        {#if SLOTS.default && SLOTS.focus}
            <Divider horizontal={false}>
                {#snippet first()}
                    <Body>
                        <slot />
                    </Body>
                {/snippet}

                {#snippet second()}
                    <div class="focus">
                        <slot name="focus" />
                    </div>
                {/snippet}
            </Divider>
        {/if}

        {#if SLOTS.focus && !SLOTS.default}
            <div class="focus">
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
                height: 100%;
                width: 100%;
                flex: 1;
            }

            &.row_mode {
                flex-direction: row;
            }   
        }
    }
</style>