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
    import Divider from "./app_frame/Divider.svelte";
    import V from "./app_frame/V.svelte";
    import RenderSides, { render_mode_str, should_show, VSide, vside_str } from "./app_frame/divider/divider";

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
    });

    onDestroy(() => {
        if (protocol) {
            protocol.websocket.close();
            protocol = null;
            protocol_ready = false;
        }
    });

    let lower_v_sides = $state(RenderSides.Both);

    let { items, ...slotProps } = $props();
</script>

<div class="root">
    <AppBar />
    <span class="hr"></span>
    <Divider horizontal={false} right_input_size={0}>
        <div class="bisect" slot="first">
            <Rail />
            <!-- {#if should_show(VSide.First, )}<V />{/if} -->

            <Divider right_input_size={0}>
                <Divider slot="first" left_input_size={0}>
                    <Frame slot="first">
                        .
                    </Frame>
    
                    <Frame slot="second" primary>
                        <CanvasXt3 slot="focus" />
                    </Frame>
                </Divider>
    
                <Frame slot="second">
                    .
                </Frame>
            </Divider>

            <V />
            <Rail />
        </div>

        <div class="bisect" slot="second">
            <Lower bind:v_sides={lower_v_sides} items={items} {...slotProps} />
        </div>
    </Divider>
    <span class="hr"></span>
    <StatusBar note={`Second: ${vside_str(VSide.Second)}; Method: ${render_mode_str(lower_v_sides)}`} />
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

        .bisect {
            display: flex;
            width: 100%;
            height: 100%;
            display: flex;
        }
    }
</style>