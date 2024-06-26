<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import AppFrame from "../components/AppFrame.svelte";
    import Protocol, {command} from "../protocol";
    import { U32_MAX, generate_bool, generate_u64, mash } from "../protocol/command";
    import Memory from "../protocol/memory";
    import Divider from "../components/app_frame/Divider.svelte";
    import Label from "../components/Label.svelte";
    import Frame from "../components/app_frame/Frame.svelte";

    let name = "";
    let greetMsg = "";

    async function greet() {
        // greetMsg = await invoke("read_memory_byte", { address: 9, translate: false });
    }

    let d = true;
</script>

<div class="root">    
    <!-- <AppFrame /> -->

    <Divider>
        <Frame slot="first">
            <Label>My Frame</Label>
        </Frame>

        <Frame slot="second">
            <Label>My Frame</Label>
        </Frame>
    </Divider>
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