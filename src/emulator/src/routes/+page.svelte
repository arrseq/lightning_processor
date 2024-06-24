<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Label from "../components/Label.svelte";
    import Button from "../components/Button.svelte";

    let name = "";
    let greetMsg = "";

    async function greet() {
        greetMsg = await invoke("read_memory_byte", { address: 9, translate: false });
    }

    let d = true;
</script>

<div class="root">    
    <Label>Welcome to the emulator</Label>
    <Label secondary={true}>Unreleased build. No distribution restrictions.</Label>
    <Label secondary={true}>Atom line. Open Source Project.</Label>
    <Button on:trigger={() => d = !d}>Initialize Studio</Button>
    <Button disabled={d} primary={true} description={"Don't install the emulator, and instead destroy it"}>Destroy</Button>
</div>

<style lang="scss">
    @import "../conf/surface.scss";
    @import "../conf/spacing.scss";

    .root {
        background: $surface__root;
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;

        // Layout
        display: flex;
        flex-direction: column;
        padding: $spacing__body;
        gap: $spacing__body;
    }
</style>