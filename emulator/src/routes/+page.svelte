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

    let name = "";
    let greetMsg = "";

    async function greet() {
        // greetMsg = await invoke("read_memory_byte", { address: 9, translate: false });
    }

    let d = true;

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
            <Frame title="Memory View">
                <Label>Tutorial information will be here soon.</Label>
                <CanvasXt3 slot="focus" />
            </Frame>
        {/snippet}

        {#snippet network_view()}
            <Frame title="Network View">
                <Label>Network Viewer is not ready.</Label>
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