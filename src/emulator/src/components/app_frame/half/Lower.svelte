<script lang="ts">
    import Label from "../../Label.svelte";
import DragBar from "../DragBar.svelte";
    import Frame from "../Frame.svelte";
    import Rail from "../Rail.svelte";
    import V from "../V.svelte";

    export let height = 100;
    export let opened = false;

    let real_height = 0;
    let snap_height = 100;
    
    let snap_width = 200;
    let first_open = true;
    let second_open = true;
    let width = 100;
    let real_width = 0;

    let frames: HTMLDivElement | null = null;

    function max_first_width(frames: HTMLDivElement) {
        return Math.min(frames.getBoundingClientRect().width, width);
    }

    function fix_state_width() {
        if (!frames) return;
        width = max_first_width(frames);

        if (real_width == 0) {
            width = 0;
        }
    }

    function fix_width() {
        if (!frames) return;
        real_width = max_first_width(frames);

        console.log(real_width);
    }

    $: {
        real_height = Math.max(snap_height, height);
        real_width = Math.max(snap_width, width);

        if (height < (snap_height / 2)) {
            real_height = 0;
        }

        if (width < (snap_width / 2)) {
            real_width = 0;
        }

        opened = real_height > 0;
        first_open = real_width > 0;

        fix_width();
    }
</script>

<div class="root" style={`flex: 0 0 ${real_height}px;`}>
    <Rail />
    {#if (first_open && second_open) || (first_open && !second_open) || (!first_open && !second_open) }<V />{/if}

    <div class="frames" class:empty={!first_open && !second_open} bind:this={frames}>
        {#if !first_open && !second_open}
            <span>
                <Label>No frames are open.</Label>
            </span>
        {/if}

        <!-- Frames -->
        {#if first_open}
            <div class="box" style={`flex: 0 0 ${real_width}px;`}>
                <Frame>
                    <Label>Frame Content</Label>
                </Frame>
            </div>
        {/if}
        
        {#if first_open || second_open}
            <DragBar vertical on:from_h={(e) => { width += e.detail; fix_width(); }} on:release={() => fix_state_width()} />
        {/if}

        {#if second_open}
            <div class="box">
                <Frame>
                    <Label>Frame Content</Label>
                </Frame>
            </div>
        {/if}
    </div>

    {#if (first_open && second_open) || (second_open && !first_open) || (!first_open && !second_open) }<V />{/if}
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
            flex: 1;
            overflow: hidden;

            &.empty {
                background: $surface__body;
            }

            .box {
                display: flex;
                flex: 1;
                height: 100%;
                overflow: hidden;
            }
        }
    }
</style>