<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import AppFrame from "../components/AppFrame.svelte";

    let name = "";
    let greetMsg = "";

    async function greet() {
        greetMsg = await invoke("read_memory_byte", { address: 9, translate: false });
    }

    let d = true;
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