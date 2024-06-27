enum RenderSides {
    Both,
    First,
    Second
}

export enum VSide {
    First,
    Second
}

export function render_mode_str(render_mode: RenderSides): string {
    if (render_mode == RenderSides.Both) return "Both";
    if (render_mode == RenderSides.First) return "First";
    return "Second";
}

export function vside_str(vside: VSide): string {
    if (vside == VSide.First) return "First";
    if (vside == VSide.Second) return "Second";
    return "Error";
}

export function should_show(self_side: VSide, render_mode: RenderSides): boolean {
    if (self_side == VSide.First && (render_mode == RenderSides.Both || render_mode == RenderSides.First)) return true;
    if (self_side == VSide.Second) return true;
    return false;
}

export default RenderSides;