<script lang="ts">
    import Label from "../../Label.svelte";
    import Divider from "../Divider.svelte";
    import RenderSides, { should_show, VSide } from "../divider/divider";
    import Frame from "../Frame.svelte";
    import Rail from "../Rail.svelte";
    import V from "../V.svelte";

    let { v_sides = $bindable(RenderSides.Both as any), items, ...slotProps } = $props();
    let current_sf = "network_view";
    let current_ss = "memory_view";

    console.log(slotProps);
</script>

<div class="root">
    <Rail />
    {#if should_show(VSide.First, v_sides)}<V />{/if}

    <Divider left_input_size={0} on:v_set={(e) => { v_sides = e.detail;  }}>
        <div class="box" slot="first">
            {#each items.filter(i => i.key == current_sf) as item}
                {@const slotProp = slotProps[item.key]}
                {@render slotProp()}
            {/each}
        </div>

        <div class="box" slot="second">
            {#each items.filter(i => i.key == current_ss) as item}
                {@const slotProp = slotProps[item.key]}
                {@render slotProp()}
            {/each}
        </div>
    </Divider>

    {#if should_show(VSide.Second, v_sides)}<V />{/if}
    <Rail />
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