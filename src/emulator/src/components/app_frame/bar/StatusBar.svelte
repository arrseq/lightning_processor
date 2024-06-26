<script lang="ts">
    import Label from "../../Label.svelte";


</script>

<div class="root">
    <Label>No events</Label>
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";

    .root {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: $surface__body;
    }
</style>