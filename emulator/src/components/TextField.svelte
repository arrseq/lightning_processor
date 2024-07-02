<script lang="ts">
    import More from "./More.svelte";

    export let disabled = false;
    export let xrt = 0;
</script>

<div class="root" class:disabled={disabled}>
    <input disabled={disabled} on:input={(e) => xrt = parseInt(e.target?.value)} />
    <span class="vr" />
    <More disabled={disabled} />
</div>

<style lang="scss">
    @import "../conf/text.scss";
    @import "../conf/surface.scss";
    @import "../conf/pixels.scss";

    .root {
        display: flex;
        background: $surface__control_secondary;
        align-items: center;
        align-self: flex-start;
        border-radius: $pixels__radius;

        span.vr {
            width: $pixels__border_control;
            height: 50%;
            background: $text__control_border_color;
        }

        input {
            background: transparent;
            outline: none;
            border: none;
            padding: 3px 14px;
            color: $text__primary_color;
            font-size: $text__primary_font_size;
            font-family: $text__font_family;
            border-radius: $pixels__radius;

            &:hover {
                background: $surface__control_secondary;
            }
        }

        &.disabled {
            background: $surface__control_primary;

            span.vr {
                height: 100%;
            }

            input {
                cursor: not-allowed;
                color: $text__secondary_color;
                background: $surface__control_primary;
            }
        }
    }
</style>