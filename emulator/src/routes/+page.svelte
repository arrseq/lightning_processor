<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import AppFrame from "../components/AppFrame.svelte";
    import Protocol, {command} from "../protocol";
    import { U32_MAX, generate_bool, generate_u64, mash } from "../protocol/command";
    import Memory from "../protocol/memory";
    import Divider from "../components/app_frame/Divider.svelte";
    import Label from "../components/Label.svelte";
    import Frame from "../components/app_frame/Frame.svelte";
    import CanvasXt3 from "../components/CanvasXT3.svelte";
    import Button from "../components/Button.svelte";
    import TextField from "../components/TextField.svelte";
    import Row from "../components/Row.svelte";
    import { onDestroy, onMount } from "svelte";
    import Flame from "../components/graph/Flame.svelte";

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

    let window_keys = [
        { key: "memory_view", rail: "ss" },
        { key: "network_view", rail: "ss" },
        { key: "settings", rail: "sf" },
        { key: "program_information", rail: "sf" }
    ]

    let windows = $state([
        ...window_keys
    ])
</script>

<div class="root">    
    <AppFrame items={windows} keys={window_keys} >
        {#snippet memory_view()}
            {#if protocol}
                <Frame title="Memory View">
                    <Flame protocol={protocol} slot="focus" />
                </Frame>
            {/if}
        {/snippet}

        {#snippet network_view()}
            <Frame title="Network View">
                <Row>
                    <Label slot="label">Account name</Label>
                    <TextField />
                </Row>

                <Row>
                    <Label slot="label">Account name</Label>
                    <TextField />
                </Row>

                <Row>
                    <Label slot="label">Account name</Label>
                    <TextField />
                </Row>

                <Row>
                    <Label slot="label">Account name</Label>
                    <TextField />
                </Row>
            </Frame>
        {/snippet}

        {#snippet settings()}
            <Frame title="Settings">
                <Label>Wait.</Label>
            </Frame>
        {/snippet}

        {#snippet program_information()}
            <Frame title="Program Fnformation">
                <Label>Developed by Atom Line.</Label>
            </Frame>
        {/snippet}
    </AppFrame>
</div>

<style lang="scss">
    @import "../conf/surface.scss";
    @import "../conf/spacing.scss";
    @import "../conf/pixels.scss";

    .root {
        background: $surface__peek;
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        gap: $pixels__border_control;

        // Layout
        display: flex;
        flex-direction: row;
    }
</style>