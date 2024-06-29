<script lang="ts">
    export let disabled = false;
</script>

<div class="root" class:disabled={disabled}>
    <span></span>
    <span></span>
    <span></span>
</div>

<style lang="scss">
    @import "../conf/text.scss";
    @import "../conf/surface.scss";
    
    .root {
        display: flex;
        padding: 4px 14px;
        align-items: center;
        justify-content: center;
        gap: 1px;
        height: 100%;
        
        span {
            width: 2px;
            height: 2px;
            background: $text__primary_color;
            border-radius: 100%;
        }

        &:hover {
            cursor: pointer;
            background: $surface__control_secondary;
        }

        &.disabled {
            cursor: not-allowed;
            background: $surface__control_primary;
            
            span {
                background: $text__secondary_color;
            }
        }
    }
</style>