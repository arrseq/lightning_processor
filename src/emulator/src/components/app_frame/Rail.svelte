<script lang="ts">
    import Item from "./rail/Item.svelte";


</script>

<div class="root">
    <Item />
    <Item />
    <Item />
    <Item />
</div>

<style lang="scss">
    @import "../../conf/surface.scss";
    
    .root {
        background: $surface__body;
        display: flex;
        height: 100%;
        flex-direction: column;
        align-self: flex-start;
        overflow: auto;
    }
</style>