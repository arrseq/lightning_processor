<script lang="ts">
    import Label from "../../Label.svelte";
    import Tabs from "./Tabs.svelte";
    import type TabInfo from "./tab_info";

    export let title;
    export let primary = false;
    export let tabs: TabInfo[] = [];
</script>

<div class="root" class:primary={primary}>
    <!-- Start half -->
    <div>
        <Label secondary>{title}</Label>

        <div class="tabs">
            <Tabs tabs={tabs} />
        </div>
    </div>

    <!-- End half -->
    <div>

    </div>
</div>

<style lang="scss">
    @import "../../../conf/surface.scss";
    @import "../../../conf/spacing.scss";

    .root {
        background: $surface__titlebar;
        display: flex;
        align-items: center;
        justify-content: space-between;
        overflow-x: auto;
        overflow-y: hidden;
        max-width: 100%;
        padding: 2px 4px;

        &.primary {
            background: $surface__body;
        }

        & > div {
            display: flex;
            align-items: center;
            gap: $spacing__body;
            max-width: 100%;

            .tabs {
                overflow-x: auto;
                overflow-y: hidden;
                max-width: 100%;
                min-width: 120px;
            }
        }
    }
</style>