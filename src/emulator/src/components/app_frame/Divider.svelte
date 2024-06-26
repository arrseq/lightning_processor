<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import DragBar from "./DragBar.svelte";

    export let horizontal = true;
    export let left_open = true;
    export let right_open = true;
    export let left_input_size = 100;
    export let snap_min = 100;
    export let snap_max = 100;

    let left_commited_size = 0;
    let self: HTMLDivElement | null = null;

    function get_size(self: HTMLDivElement) {
        let bound = self.getBoundingClientRect();
        if (horizontal) { return bound.width; }
        return bound.height;
    }

    // Get the left input size that is constrained to the maximum width. 
    function get_max_bound(self: HTMLDivElement) {
        return Math.min(get_size(self), left_input_size);
    }

    // Process the input size. This is data that came from the drag trigger.
    function process_input() {
        if (!self) return;
        // Make sure size doesn't exit window. Constrain it to the window dimensions.
        left_commited_size = get_max_bound(self);

        // Setup the upper snap.
        let upper_snap_start = get_size(self) - snap_max;
        left_commited_size = Math.min(upper_snap_start, left_commited_size);
        if (left_input_size > upper_snap_start + (snap_max / 2)) {
            left_commited_size = get_size(self);
        }

        // Setup the lower snap.
        left_commited_size = Math.max(snap_min, left_commited_size);

        if (left_input_size < (snap_min / 2)) {
            left_commited_size = 0;
        }

        // If its either equal or larger than 0. The equality part allows for the window to snap start if the window is
        // not large enough.
        left_open = left_commited_size >= 0;
        right_open = left_commited_size < upper_snap_start;
    }

    let resize_observer: ResizeObserver | null;

    function on_resize() {
        if (!self) return;
        // Account for the max width changing. Updating the input to match either the last commited value or the width
        // of this divider. If the width of the divider is less, then the left window must collapse.
        left_input_size = Math.min(left_commited_size, get_size(self));
        process_input();
    }

    function destroy_resize() {
        if (resize_observer) {
            resize_observer.disconnect();
            resize_observer = null;
        }
    }

    function create_resize() {
        if (resize_observer) {
            destroy_resize();
        }

        if (!self) return;
        resize_observer = new ResizeObserver(on_resize);
        resize_observer.observe(self);
    }

    onMount(() => {
        process_input();
        create_resize();
    });

    onDestroy(() => {
        destroy_resize();
    });
</script>

<div class="root" bind:this={self} class:horizontal={horizontal}>
    <div class="box" style={`flex: 0 0 ${left_commited_size}px`}>
        <slot name="first" />
    </div>

    {#if left_open || right_open}
        <DragBar vertical={horizontal} on:from_h={(e) => {
            if (!horizontal) return;
            left_input_size += e.detail;
            process_input();
        }} on:from_v={(e) => {
            if (horizontal) return;
            left_input_size -= e.detail;
            process_input();
        }} on:release={() => {
            if (!self) return;
            // If the user closed the panel, then set the input to 0 incase snapping is not 0. This is because if the window
            // snaps close, it makes a jump which the drag bar is not aware of. We need to make sure new drags start at 0 
            // since the panel closed.
            //
            // This drag bar doesnt need to handle both being closed because the dragbar will be innaccessable during
            // that state.
            if (!left_open) { left_input_size = 0; }
            if (!right_open) { left_input_size = get_size(self); }
        }} />
    {/if}

    <div class="box">
        <slot name="second" />
    </div>
</div>

<style lang="scss">
    .root {
        display: flex;
        flex: 1;
        overflow: hidden;
        flex-direction: column;

        &.horizontal {
            flex-direction: row;
            flex: 1;
            width: 100%;
        }

        &:not(&.horizontal) {
            height: 100%;
        }

        .box {
            display: flex;
            flex: 1;
            overflow: hidden;
        }
    }
</style>