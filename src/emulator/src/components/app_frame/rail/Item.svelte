<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Label from "../../Label.svelte";

    export let selected = false;
    let dispatch = createEventDispatcher();
</script>

<div class="root" class:selected={selected} on:click={() => dispatch("select")}>
    <Label vertical>
        <slot />
    </Label>
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";
    @import "../../../conf/text.scss";

    .root {
        display: flex;
        align-self: flex-start;

        &:hover {
            background: $surface__control_secondary;
        }

        &.selected {
            box-shadow: 0 0 0 1px $text__brand_color inset;
        }
    }
</style>