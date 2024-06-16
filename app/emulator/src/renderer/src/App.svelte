<script lang="ts">
    import { Button, Field } from "atln-kit";
  import { onMount } from "svelte"

    onMount(() => {
        const canvas = document.querySelector("canvas")
        const ctx = canvas.getContext("2d");;

        const data = ctx.createImageData(canvas.width, canvas.height);
    })  
</script>

<Button>None</Button>
<Field />

<h3>Active Region</h3>
<canvas width="500" height="300" />

<style lang="scss">
    canvas {
        border: 1px solid red;
    }
</style>