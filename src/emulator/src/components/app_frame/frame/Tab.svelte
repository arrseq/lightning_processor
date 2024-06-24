<script lang="ts">
    import Label from "../../Label.svelte";
    import More from "../../More.svelte";

    export let label = "Tab label not initialized";
    export let description = "";
    export let disabled = false;
</script>

<div class="root" class:disabled={disabled} title={description}>
    <div class="label">
        <Label secondary={disabled} highlight={description.length > 0}>{label}</Label>
    </div>
    <span />
    <More disabled={disabled} />
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";
    @import "../../../conf/text.scss";
    @import "../../../conf/pixels.scss";
    @import "../../../conf/anime.scss";

    .root {
        display: flex;
        transition: $anime__transition_out;
        box-shadow: 0 0 0 0 transparent inset;
        align-items: center;
        background: $surface__control_secondary;

        // Vertical rule.
        & > span {
            width: $pixels__border_control;
            height: 50%;
            background: $text__control_border_color;
        }

        &:hover:not(&.disabled) {
            box-shadow: 0 0 0 $pixels__border_control $text__control_border_color inset;
            transition: $anime__transition_in;
        }

        &:not(&.disabled) .label:hover {
            background: $surface__control_secondary;
            transition: $anime__transition_in;
        }

        &.disabled {
            cursor: not-allowed;
            
            .label {
                background: $surface__control_primary;
            }

            & > span {
                height: 100%;
            }
        }
    }
</style>