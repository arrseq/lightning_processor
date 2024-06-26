<script lang="ts">
    import Label from "../../Label.svelte";
import DragBar from "../DragBar.svelte";
    import Rail from "../Rail.svelte";
    import V from "../V.svelte";

    export let height = 100;
    export let opened = false;

    let real_height = 0;
    let snap_height = 100;

    let first_open = false;
    let second_open = false;
    let first_width = 100;

    $: {
        real_height = Math.max(snap_height, height);

        if (height < (snap_height / 2)) {
            real_height = 0;
        }

        opened = real_height > 0;
    }
</script>

<div class="root" style={`flex: 0 0 ${real_height}px;`}>
    <Rail />
    {#if !first_open && !second_open}
        <V />
    {/if}
    <div class="frames" class:empty={!first_open && !second_open}>
        {#if !first_open && !second_open}
            <span>
                <Label>No frames are open.</Label>
            </span>
        {/if}
    </div>
    {#if !first_open && !second_open}
        <V />
    {/if}
    <Rail />
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";

    .root {
        width: 100%;
        overflow: auto;
        display: flex;
        justify-content: space-between;

        .frames {
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            background: $surface__body;
            flex: 1;
        }
    }
</style>