<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "./Button.svelte";
    import CanvasXt3 from "./CanvasXT3.svelte";
    import Label from "./Label.svelte";
    import TextField from "./TextField.svelte";
    import AppBar from "./app_frame/AppBar.svelte";
    import DragBar from "./app_frame/DragBar.svelte";
    import Frame from "./app_frame/Frame.svelte";
    import Rail from "./app_frame/Rail.svelte";

    let bottom_height = 320;
    let start_width = 320;
    let end_width = 320;
    let b_start_width = 320;

    let rt = false;
    let xrt = 100;

    async function render(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
        let buf = ctx.createImageData(canvas.width, canvas.height);
        let r_buf = await invoke("get_red_noise", { width: canvas.width, height: canvas.height }) as number[];
        let r_u8_buf = new Uint8Array(r_buf);

        buf.data.set(r_u8_buf);

        ctx.putImageData(buf, 0, 0);
    } 
</script>

<div class="root">
    <AppBar />
    <span class="hr"></span>
    <div>
        <Rail />
        <div class="main">
            <div class="box" style={`min-width: ${start_width}px; max-width: ${start_width}px;`}>
                <Frame title="Main Area">
                    <Label>CanvasXT3 Renderer.</Label>
                    <Label secondary>Warning: Canvas rendering may reduce performance.</Label>
                    <br />
                    <Label secondary highlight="Enter a number. The number to square to generate the red square in the rendering region.">Area Factor</Label>
                    <TextField bind:xrt={xrt} />
                    <TextField bind:xrt={xrt} disabled />
                    <Button primary={rt} on:trigger={() => rt = !rt}>Real Time Update</Button>
                </Frame>
            </div>
            <DragBar vertical on:from_h={(offset) => start_width -= offset.detail} />
            <div class="span">
                <Frame title="Main Area" primary tabs={[
                    { label: "Unnamed Emulation", description: "This emulator panicked!", disabled: true },
                    { label: "My Emulation", description: "", disabled: false }
                ]}>
                    <CanvasXt3 slot="focus" on:open_render={async (xt3) => await render(xt3.detail.context, xt3.detail.canvas)} on:resize={async (s) => {
                        // await invoke("set_buffer_size", { width: 10, height: 10 })
                    }} match_resolution={true} resolution={[1000, 100]} />
                </Frame>
            </div>
            <DragBar vertical  on:from_h={(offset) => end_width += offset.detail} />
            <div class="box" style={`min-width: ${end_width}px; max-width: ${end_width}px;`}>
                <Frame title="Main Area">A</Frame>
            </div>
        </div>
        <Rail />
    </div>

    <DragBar on:from_v={(offset) => bottom_height += offset.detail } />

    <div style={`min-height: ${bottom_height}px; max-height: ${bottom_height}px;`}>
        <Rail />
        <div class="main">
            <div class="box" style={`min-width: ${b_start_width}px; max-width: ${b_start_width}px;`}>
                <Frame title="Main Area">A</Frame>
            </div>
            <DragBar vertical on:from_h={(offset) => b_start_width -= offset.detail} />
            <div class="span">
                <Frame title="Main Area">A</Frame>
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

        span.hr {
            height: $pixels__border_control;
        }

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