<script lang="ts">
    import Label from "../Label.svelte";


</script>

<div class="root" data-tauri-drag-region>
    <Label>Atom Line: Processor Emulator</Label>
</div>

<style lang="scss">
    @import "../../conf/surface.scss";

    .root {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: $surface__body;
    }
</style>