<div>
    <slot />
</div>

<style lang="scss">
    @import "../lib/config.scss";

    div {
        background: $panel_body;
        width: 100vw;
        height: 100vh;
        position: fixed;
        top: 0;
        left: 0;
        padding: 20px;
   }
</style>