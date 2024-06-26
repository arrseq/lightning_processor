<script lang="ts">
    import { createEventDispatcher, onDestroy, onMount } from "svelte";

    export let vertical = false;

    let dragging_busy = false;
    let origin = [0, 0];
    let mouse_pos = [0, 0];

    const dispatch = createEventDispatcher();

    function drag_start(event: MouseEvent) {
        event.preventDefault();
        origin = [event.clientX, event.clientY];
        dragging_busy = true;

        window.addEventListener("mousemove", window_mouse_move);
    }

    function window_mouse_move(event: MouseEvent) {
        event.preventDefault();
        mouse_pos = [event.clientX, event.clientY];
        dispatch("from_v", origin[1] - mouse_pos[1]);
        dispatch("from_h", -(origin[0] - mouse_pos[0]));
        origin = mouse_pos;
    }

    function remove_mouse_listener() {
        window.removeEventListener("mousemove", window_mouse_move);
        dragging_busy = false;
    }

    function window_on_mouse_up() {
        remove_mouse_listener();
        dispatch("release");
    }

    onMount(() => {
        window.addEventListener("mouseup", window_on_mouse_up);
    });

    onDestroy(() => {
        window.removeEventListener("mouseup", window_on_mouse_up);

        if (dragging_busy) {
            remove_mouse_listener();
        }
    });
</script>

<div on:mousedown={(e) => drag_start(e)} class="root" class:vertical={vertical} />

<style lang="scss">
    @import "../../conf/pixels.scss";
    @import "../../conf/text.scss";
    @import "../../conf/anime.scss";

    .root {
        width: 100%;
        height: $pixels__border_control;
        position: relative;

        &::after {
            content: "";
            display: flex;
            position: absolute;
            width: 100%;
            height: 8px;
            cursor: n-resize;
            user-select: none;
            transform: translate(0%, -50%);
        } 

        &.vertical {
            width: $pixels__border_control;
            height: 100%;

            &::after {
                transform: translate(-50%, 0%);
                width: 8px;
                height: 100%;
                cursor: e-resize;
            }
        }
    }
</style>