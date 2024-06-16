<script lang="ts">
    export let primary = false;
</script>

<button class={primary ? "primary" : ""}>
    <slot />
</button>

<style lang="scss">
    @import "./config.scss";

    button {
        border: 1px solid $border_0;
        background: $layer_0;
        padding: $c_pad;
        color: $c_text_1;
        font-size: $font_size;
        border-radius: $radius;
        font-family: $family;
        min-width: 120px;
        cursor: pointer;

        &.primary {
            background: $accent;
            color: $accent_text;
        }

        &:hover:not(.primary) {
            background: $layer_1;

            &:active {
                color: $c_text_0;
            }
        }
    }
</style>