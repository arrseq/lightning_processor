<script lang="ts">
    import CanvasXt3 from "../../CanvasXT3.svelte";
    import Rail from "../Rail.svelte";
    import V from "../V.svelte";


</script>

<div class="root">
    <Rail />
    <V />
    
    <CanvasXt3 />

    <V />
    <Rail />
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";

    .root {
        flex: 1 0 0;
        width: 100%;
        background: $surface__body;
        overflow: auto;
        display: flex;
    }
</style>