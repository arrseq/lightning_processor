<script lang="ts">
    import { createEventDispatcher, onDestroy, onMount } from "svelte";
    import Label from "./Label.svelte";

    // Set the canvas resolution to its width and height automatically.
    export let match_resolution = true;
    export let resolution: [number, number] = [0, 0];

    let self: HTMLDivElement | null = null;
    let observer: ResizeObserver | null = null;
    let context: CanvasRenderingContext2D | null = null;
    let canvas: HTMLCanvasElement | null = null;
    let msg: HTMLSpanElement | null = null;
    let loop_id: any = null;
    let msg_timeout: any = null;
    let message_data = "";

    const dispatch = createEventDispatcher();

    function on_resize() {
        if (!msg || !self || !canvas) return;

        canvas.width = resolution[0];
        canvas.height = resolution[1];

        if (match_resolution) {
            let rect = self.getBoundingClientRect();
            canvas.width = rect.width;
            canvas.height = rect.height; 
        }

        dispatch("resize", { width: canvas.width, height: canvas.height });

        let px_count = new ImageData(canvas.width, canvas.height).data.length;
        message_data = `Canvas: Sized updated to ${canvas.width} by ${canvas.height} pixels. Image buffer reports ${px_count} pixel cap`;
        msg.classList.remove("hidden");

        clearTimeout(msg_timeout);
        msg_timeout = setTimeout(() => {
            if (!msg) return;
            msg.classList.add("hidden");
        }, 1500);
    }

    function render() {
        if (!context || !canvas) return;
        dispatch("open_render", { context, canvas });
        loop_id = window.requestAnimationFrame(render);
    }

    onMount(() => {
        if (!self || !canvas) return;

        // Setup render loop
        context = canvas.getContext("2d");
        window.requestAnimationFrame(render);

        // Resize listener for resolution matching.
        if (observer) {
            observer.disconnect();
        }

        observer = new ResizeObserver(on_resize);
        observer.observe(self);

        dispatch("resize", { width: canvas.width, height: canvas.height, canvas });
    });

    onDestroy(() => {
        window.cancelAnimationFrame(loop_id);

        if (observer) {
            observer.disconnect();
            observer = null;
        }
    })
</script>

<!-- Canvas Scaling Manager -->
<div class="root" bind:this={self}>
    <canvas bind:this={canvas} />

    <span class="message hidden" bind:this={msg}>
        <Label>{message_data}</Label>
    </span>
</div>

<style lang="scss">
    @import "../conf/anime.scss";
    @import "../conf/surface.scss";

    .root {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        align-self: flex-start;
        flex: 1;
        position: relative;

        .message {
            position: absolute;
            top: 0;
            left: 0;
            transition: $anime__transition_in;
            width: 100%;
            background: $surface__root;
            display: flex;
            align-items: center;
            justify-content: flex-start;

            &.hidden {
                pointer-events: none;
                opacity: 0;
                transition: $anime__transition_out;
            }
        }

        canvas {
            width: 100%;
            height: 100%;
            min-width: 0;
            min-height: 0;
            position: absolute;
            background: #000;
        }
    }
</style>