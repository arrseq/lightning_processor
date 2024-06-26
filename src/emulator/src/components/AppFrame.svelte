<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "./Button.svelte";
    import CanvasXt3 from "./CanvasXT3.svelte";
    import Label from "./Label.svelte";
    import TextField from "./TextField.svelte";
    import AppBar from "./app_frame/bar/AppBar.svelte";
    import DragBar from "./app_frame/DragBar.svelte";
    import Frame from "./app_frame/Frame.svelte";
    import Rail from "./app_frame/Rail.svelte";
    import Protocol from "../protocol";
    import { onDestroy, onMount } from "svelte";
    import { Commands, generate_u64, mash } from "../protocol/command";
    import Upper from "./app_frame/half/Upper.svelte";
    import Lower from "./app_frame/half/Lower.svelte";
    import StatusBar from "./app_frame/bar/StatusBar.svelte";

    let frames_window: HTMLDivElement | null = null;

    let lower_height = 100;
    let lower_height_valid = 0;
    let lower_opened = false;

    let protocol: Protocol | null;
    let protocol_ready = false;
    let protocol_queue: (() => void)[] = [];

    function use_protocol(once_ready: (protocol: Protocol) => void): Promise<void> {
        return new Promise((resolve) => {
            let handle_logic = () => {
                once_ready(protocol as any).then(() => resolve());
            };

            if (!protocol_ready) {
                protocol_queue.push(handle_logic);
                return;
            }

            handle_logic();
        });
    }

    function max_lower_height(frames_window: HTMLDivElement) {
        return Math.min(frames_window.getBoundingClientRect().height, lower_height);
    }

    function fix_height() {
        if (!frames_window) return;
        lower_height_valid = max_lower_height(frames_window);
    }

    function fix_state_height() {
        if (!frames_window) return;
        lower_height = max_lower_height(frames_window);
    }

    let app_frame: HTMLDivElement | null = null;
    let resize_observer: ResizeObserver | null = null;

    function app_frame_resize() {
        console.log("Win resize");
        fix_state_height();
        fix_height();
    }

    $: {
        fix_height();
    }

    onMount(() => {
        if (!protocol) {
            protocol = new Protocol();

            protocol.on_close_listener = () => {
                protocol = null;
                protocol_ready = false;
            }

            protocol.on_open_listener = () => {
                protocol_ready = true;
                protocol_queue.forEach((waiter) => waiter());
            }
        }

        if (app_frame) {
            if (resize_observer) {
                resize_observer.disconnect();
                resize_observer = null;
            }

            resize_observer = new ResizeObserver(app_frame_resize);
            resize_observer.observe(app_frame);
        }
    });

    onDestroy(() => {
        if (protocol) {
            protocol.websocket.close();
            protocol = null;
            protocol_ready = false;
        }

        if (resize_observer) {
            resize_observer.disconnect();
            resize_observer = null;
        }
    });
</script>

<div class="root" bind:this={app_frame}>
    <AppBar />
    <span class="hr"></span>
    <div class="frames" bind:this={frames_window}>
        <Upper />
        <DragBar on:from_v={(e) => { lower_height += e.detail; fix_height(); }} on:release={() => fix_state_height()} />
        <Lower height={lower_height_valid} bind:opened={lower_opened} />
    </div>
    {#if lower_opened}
        <span class="hr"></span>
    {/if}
    <StatusBar />
</div>

<style lang="scss">
    @import "../conf/spacing.scss";
    @import "../conf/surface.scss";
    @import "../conf/pixels.scss";

    .root {
        background: $surface__peek;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        padding-top: $pixels__border_control;

        span.hr {
            height: $pixels__border_control;
        }

        .frames {
            display: flex;
            flex-direction: column;
            flex: 1;
            overflow: hidden;
        }
    }
</style>