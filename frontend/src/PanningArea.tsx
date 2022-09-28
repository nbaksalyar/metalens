import { NavState } from "interfaces";
import React, { Dispatch, SetStateAction, useCallback, useEffect, } from "react";

interface PanningAreaProps {
    navState: NavState;
    setNavState: Dispatch<SetStateAction<NavState>>;
    updateArrows: any;
    // children?: React.ReactNode;
    // onMouseDown: (event: React.MouseEvent<HTMLDivElement>) => void;
    // onMouseUp: (event: React.MouseEvent<HTMLDivElement>) => void;
    // onMouseMove: (event: React.MouseEvent<HTMLDivElement>) => void;
}

const PanningArea: React.FC<PanningAreaProps> = (props) => {

    // FIXME: UGLY HACK to update arrows on css transitions!
    const uglyHack = () => {
        for (var i = 0; i < 20; i++) {
            setTimeout(props.updateArrows, i * 15)
        }
    }

    useEffect(() => {
        const cancelWheel = (e: any) => e.preventDefault();
        const cancelBrowserZoom = (e: any) => {
            // preventing default browser zoom-in and zoom-out hotkeys
            if (e.metaKey === true) {
                if (e.key == "-" || e.key == "0" || e.key == "=") {
                    e.preventDefault();
                }
            };
        };
        document.body.addEventListener('wheel', cancelWheel, {passive: false});
        document.body.addEventListener('keydown', cancelBrowserZoom, {passive: false});

        return () => {
            document.body.removeEventListener('wheel', cancelWheel);
            document.body.removeEventListener('keydown', cancelBrowserZoom);
        }
    }, []);

    const handleShortcuts = useCallback((e) => {

        if (e.metaKey === true || e.ctrlKey === true) {
            if (e.key == "=") {
                if (props.navState.zoom + 0.1 < 1.2) {
                    props.setNavState((prevState) => ({
                        ...prevState,
                        zoom: prevState.zoom + 0.2,
                    }));
                }
            } else if (e.key == "-") {
                if (props.navState.zoom - 0.1 > 0.5) {
                    props.setNavState((prevState) => ({
                        ...prevState,
                        zoom: prevState.zoom - 0.2,
                    }));
                }
            } else if (e.key == "0") {
                props.setNavState((prevState) => ({
                    ...prevState,
                    zoom: 1,
                }));
            }
            uglyHack();
        } else if (e.key == "ArrowLeft") {
            props.setNavState((prevState) => ({
                ...prevState,
                stepOffsetX: prevState.stepOffsetX + 200,
            }));
            uglyHack();
        } else if (e.key == "ArrowRight") {
            props.setNavState((prevState) => ({
                ...prevState,
                stepOffsetX: prevState.stepOffsetX - 200,
            }));
            uglyHack();
        } else if (e.key == "ArrowUp") {
            props.setNavState((prevState) => ({
                ...prevState,
                stepOffsetY: prevState.stepOffsetY + 200,
            }));
            uglyHack();
        } else if (e.key == "ArrowDown") {
            props.setNavState((prevState) => ({
                ...prevState,
                stepOffsetY: prevState.stepOffsetY - 200,
            }));
            uglyHack();
        }
    }, [props.navState]);

    useEffect(() => {
        document.addEventListener('keydown', handleShortcuts);

        return () => {
            document.removeEventListener('keydown', handleShortcuts);
        };
    }, [handleShortcuts]);

    const formatBgStyle = useCallback(() => {
        const zoomFactor = (props.navState.zoom/0.5) * 40;
        const computedX = props.navState.offsetX * props.navState.zoom;
        const computedY = props.navState.offsetY * props.navState.zoom;
        let computedDot = 2 + props.navState.zoom-1;
        const bgi = `radial-gradient(rgba(255,255,255,0.04) ` +
                        `${computedDot}px, transparent ${computedDot}px), ` +
                    `radial-gradient(rgba(255,255,255,0.02) ` +
                        `${computedDot}px, transparent ${computedDot}px), ` +
                    `radial-gradient(rgba(255,255,255,0.02) ` +
                        `${computedDot}px, transparent ${computedDot}px)`
        const bgp = `${computedX}px ${computedY}px, `+
                    `${computedX}px ${(computedY) + zoomFactor/2}px, `+
                    `${computedX + zoomFactor/2}px ${computedY}px`
        const bgs = zoomFactor;
        return {
            backgroundImage: bgi,
            backgroundPosition: bgp,
            backgroundSize: `${bgs}px ${bgs}px, ${bgs}px ${bgs}px`,
        }
    }, [props.navState]);

    const startPanning = useCallback((e: any) => {
        e.preventDefault();
        props.setNavState((prevState) => ({
            ...prevState,
            offsetX: props.navState.offsetX,
            offsetY: props.navState.offsetY,
            initialX: e.clientX - props.navState.offsetX,
            initialY: e.clientY - props.navState.offsetY,
            isPanning: true,
        }));
    }, [props.navState]);

    const stopPanning = useCallback((e: any) => {
        e.preventDefault();
        props.setNavState((prevState) => ({
            ...prevState,
            isPanning: false,
        }));
    }, [props.navState]);

    const handlePanning = (e: any) => {
        e.preventDefault();
        if (props.navState.isPanning) {
            props.setNavState((prevState) => ({
                ...prevState,
                offsetX: e.clientX - prevState.initialX,
                offsetY: e.clientY - prevState.initialY,
            }));
        }
    }

    const zoomIn = (e: any) => {
        if (e.metaKey === true && props.navState.zoom >= 0.5) {
            props.setNavState((prevState) => ({
                ...prevState,
                zoom: prevState.zoom - 0.25,
            }));
            uglyHack();
        } else if (e.metaKey === false && props.navState.zoom <= 1) {
            props.setNavState((prevState) => ({
                ...prevState,
                zoom: prevState.zoom + 0.25,
            }));
            uglyHack();
        }
    }

    const handleGesture = (e: any) => {
        e.preventDefault();
        if (e.metaKey == true || e.ctrlKey == true) {
            let deltaY = e.deltaY;

            if (props.navState.zoom >= 1.2 && e.deltaY > 0) {
                return;
            } else if (props.navState.zoom < 0.5 && e.deltaY < 0) {
                return;
            }
            let amount = 0;
            if (e.deltaY > 1) {
                amount = 0.025;
            } else if (e.deltaY < -1) {
                amount = -0.025;
            }
            props.setNavState((prevState) => ({
                ...prevState,
                zoom: prevState.zoom + amount,
            }));
            // FIXME: UGLY HACK!
            for (var i = 0; i < 6; i++) setTimeout(props.updateArrows, i * 60);
        } else {
            props.setNavState((prevState) => ({
                ...prevState,
                offsetX: prevState.offsetX - Math.floor(e.deltaX / 1),
                offsetY: prevState.offsetY - Math.floor(e.deltaY / 1),
            }));
        }
    }

    return (
        <div
            style={formatBgStyle()}
            className='panning-field'
            onMouseDown={startPanning}
            onMouseUp={stopPanning}
            onMouseMove={handlePanning}
            onMouseOut={stopPanning}
            onWheel={handleGesture}
            onDoubleClick={zoomIn}

        />
    );
}

export default PanningArea;