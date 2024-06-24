<script>
    import { createEventDispatcher } from "svelte";
    import Label from "./Label.svelte";

    export let primary = false;
    export let disabled = false;
    export let description = "";

    const dispatch = createEventDispatcher();
</script>

<button 
    class="root" 
    class:primary={primary && !disabled} 
    class:disabled={disabled}
    title={description}
    on:click={() => dispatch("trigger")}
>
    <Label active={primary && !disabled} highlight={description.length > 0}>
        <slot />
    </Label>
</button>

<style lang="scss">
    @import "../conf/surface.scss";
    @import "../conf/pixels.scss";
    @import "../conf/text.scss";
    @import "../conf/anime.scss";

    .root {
        border: $pixels__border_control solid $text__control_border_color;
        display: flex;
        align-self: flex-start;
        border-radius: $pixels__radius;
        background: $surface__control_secondary;
        transition: $anime__transition_out;

        &.primary {
            background: $text__brand_color;
            transition: $anime__transition_in;
        }

        &:hover:not(&.disabled) {
            background: $surface__control_primary;
            cursor: pointer;
            transition: $anime__transition_in;

            &.primary {
                background: $text__brand_color_alt;
            }
        }

        &.disabled {
            opacity: $text__disabled_opacity;
            transition: $anime__transition_in;
            cursor: not-allowed;
        }
    }
</style>