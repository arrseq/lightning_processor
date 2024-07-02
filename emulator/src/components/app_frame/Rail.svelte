<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Item from "./rail/Item.svelte";

    let { keys = [] as any[], selected = $bindable("") } = $props();
    let dispatch = createEventDispatcher();
</script>

<div class="root">
    {#each keys as key}
        <Item selected={key == selected} on:select={() => {
            if (selected == key) {
                selected = "";
            } else {
                selected = key;
                dispatch("open", selected);
            }

            dispatch("select", selected);
        }}>{key}</Item>
    {/each}
</div>

<style lang="scss">
    @import "../../conf/surface.scss";
    @import "../../conf/pixels.scss";
    
    .root {
        background: $surface__body;
        display: flex;
        height: 100%;
        flex-direction: column;
        align-self: flex-start;
        overflow: auto;
        width: $pixels__label_height;
        padding: 2px;
    }
</style>