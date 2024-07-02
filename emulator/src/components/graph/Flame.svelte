<script lang="ts">
    import type Protocol from "../../protocol";
    import CanvasXt3 from "../CanvasXT3.svelte";

    export let protocol: Protocol;

    function render(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
        console.log("Rendering Graph Frame");
    }
</script>

<CanvasXt3 match_resolution={true} on:open_render={e => render(e.detail, e.detail)} />