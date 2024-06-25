<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import AppFrame from "../components/AppFrame.svelte";
    import Protocol, {command} from "../protocol";
    import { U32_MAX, generate_bool, generate_u64, mash } from "../protocol/command";

    let name = "";
    let greetMsg = "";

    async function greet() {
        // greetMsg = await invoke("read_memory_byte", { address: 9, translate: false });
    }

    let d = true;

    $: {
        let pro = new Protocol();

        pro.on_open_listener = async () => {
            let response = await pro.send_raw_command(command.Commands.Memory__ReadByteFrame, 0, mash([ generate_u64(0, 0), generate_bool(false) ]));
            console.log("R", response)
        };
    }
</script>

<div class="root">    
    <AppFrame />
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