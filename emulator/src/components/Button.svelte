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
    on:click={() => dispatch("trigger")}
>
    <Label active={primary && !disabled} highlight={description}>
        <slot />
    </Label>
</button>

<style lang="scss">
    @import "../conf/surface.scss";
    @import "../conf/pixels.scss";
    @import "../conf/text.scss";
    @import "../conf/anime.scss";

    .root {
        // box-shadow: 0 0 0 $pixels__border_control $text__control_border_color inset;
        border: none;
        display: flex;
        align-self: flex-start;
        border-radius: $pixels__radius;
        background: $surface__control_secondary;
        transition: $anime__transition_out;
        min-width: 120px;
        justify-content: center;
        align-items: center;

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