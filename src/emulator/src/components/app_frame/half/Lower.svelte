<script lang="ts">
    import Label from "../../Label.svelte";
    import Divider from "../Divider.svelte";
    import RenderSides, { should_show, VSide } from "../divider/divider";
    import Frame from "../Frame.svelte";
    import Rail from "../Rail.svelte";
    import V from "../V.svelte";
    import NoPanel from "./NoPanel.svelte";
    import type { Api as DividerApi } from "../divider/divider";

    let { v_sides = $bindable(RenderSides.Both as any), keys = [] as string[], items, ...slotProps } = $props();
    let current_sf = $state("");
    let current_ss = $state("");

    let sf_width = $state(0);

    let divider: DividerApi | null = $state(null);

    let first_soft_close = $state(false);
    let second_soft_close = $state(false);
</script>

<div class="root">
    <Rail keys={keys.filter(key => key.rail == "sf").map(key => key.key)} bind:selected={current_sf} on:open={() => { divider?.open_left(); }} />
    {#if should_show(VSide.First, v_sides)}<V />{/if}
    
    <Divider bind:x={divider} left_input_size={0} on:v_set={(e) => { v_sides = e.detail; }} on:left_closed={() => current_sf = ""} on:right_closed={() => current_ss = ""}>
        {#snippet first()}
            <div class="box">
                {#each items.filter(i => i.key == current_sf) as item}
                    {@const slotProp = slotProps[item.key]}
                    {@render slotProp()}
                {/each}

                {#if items.filter(i => i.key == current_sf).length == 0}
                    <NoPanel />
                {/if}
            </div>
        {/snippet}

        {#snippet second()}
            <div class="box">
                {#each items.filter(i => i.key == current_ss) as item}
                    {@const slotProp = slotProps[item.key]}
                    {@render slotProp()}
                {/each}

                {#if items.filter(i => i.key == current_ss).length == 0}
                    <NoPanel />
                {/if}
            </div>
        {/snippet}
    </Divider>

    {#if should_show(VSide.Second, v_sides)}<V />{/if}
    <Rail keys={keys.filter(key => key.rail == "ss").map(key => key.key)} bind:selected={current_ss} on:open={() => { divider?.open_right(); }} />
</div>

<style lang="scss">
    .root {
        display: flex;
        flex: 1;
        width: 100%;
        height: 100%;

        .box {
            display: flex;
            width: 100%;
            height: 100%;
            flex: 1;   
        }
    }
</style>