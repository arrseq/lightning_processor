<script lang="ts">
    import Label from "../../Label.svelte";
</script>

<div class="root">
    <Label vertical>Item</Label>
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";

    .root {
        &:hover {
            background: $surface__control_secondary;
        }
    }
</style>