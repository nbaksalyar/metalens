interface NavState {
    offsetX: number;
    offsetY: number;
    stepOffsetX: number;
    stepOffsetY: number;
    initialX: number;
    initialY: number;
    zoom: number;
    isPanning: boolean;
}

export type { NavState, };