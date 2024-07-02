<div class="root">
    <div class="label">
        <slot name="label" />
    </div>

    <div class="input">
        <slot />
    </div>
</div>

<style lang="scss">
    @import "../conf/surface.scss";

    .root {
        display: flex;
        gap: 4px;
    }
</style>