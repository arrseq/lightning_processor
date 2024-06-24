<script lang="ts">
    import DragBar from "./app_frame/DragBar.svelte";
import Frame from "./app_frame/Frame.svelte";
    import Rail from "./app_frame/Rail.svelte";

    let bottom_height = 320;
    let start_width = 320;
    let end_width = 320;
    let b_start_width = 320;
</script>

<div class="root">
    <div>
        <Rail />
        <div class="main">
            <div class="box" style={`min-width: ${start_width}px; max-width: ${start_width}px;`}>
                <Frame title="Main Area" />
            </div>
            <DragBar vertical on:from_h={(offset) => start_width -= offset.detail} />
            <div class="span">
                <Frame title="Main Area" primary />
            </div>
            <DragBar vertical  on:from_h={(offset) => end_width += offset.detail} />
            <div class="box" style={`min-width: ${end_width}px; max-width: ${end_width}px;`}>
                <Frame title="Main Area" />
            </div>
        </div>
        <Rail />
    </div>

    <DragBar on:from_v={(offset) => bottom_height += offset.detail } />

    <div style={`min-height: ${bottom_height}px; max-height: ${bottom_height}px;`}>
        <Rail />
        <div class="main">
            <div class="box" style={`min-width: ${b_start_width}px; max-width: ${b_start_width}px;`}>
                <Frame title="Main Area" />
            </div>
            <DragBar vertical on:from_h={(offset) => b_start_width -= offset.detail} />
            <div class="span">
                <Frame title="Main Area" />
            </div>
        </div>
        <Rail />
    </div>
</div>

<style lang="scss">
    @import "../conf/spacing.scss";
    @import "../conf/surface.scss";
    @import "../conf/pixels.scss";

    .root {
        background: $surface__peek;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        padding-top: $pixels__border_control;

        & > div {
            display: flex;
            gap: $pixels__border_control;
            width: 100%;
            flex: 1;
            justify-content: space-between;

            .main {
                display: flex;
                flex: 1;
                height: 100%;

                .span {
                    flex: 1;
                    display: flex;
                    height: 100%;
                }

                .box {
                    display: flex;
                    height: 100%;
                }
            }
        }
    }
</style>