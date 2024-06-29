<script>
    import Label from "../../Label.svelte";

</script>
<div class="root">
    <Label>No Panel</Label>
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";

    .root {
        display: flex;
        width: 100%;
        height: 100%;
        flex: 1;
        background: $surface__root;
        align-items: center;
        justify-content: center;
    }
</style>