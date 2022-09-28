import React, { useState } from 'react';
import ReactTooltip from "react-tooltip";

interface IconProps {
    className: string; // "some-thing-icon"
    width: number; // 8
    height: number; // 8
    hint?: string;
    fill: string; // #DADADA
    hoverFill: string; // #D9BA3E
    children: JSX.Element; // An SVG Path element
    onClick: (event: React.MouseEvent<SVGSVGElement>) => void;
}

const BaseSvgIcon: React.FC<IconProps> = (props) => {

    const [hover, setHover] = useState(false);
    const viewBox = `0 0 ${props.width} ${props.height}`;
    const color = hover ? props.hoverFill : props.fill;
    const tooltip_id="icon-tooltip" + props.className;

    return (
        <>
            <ReactTooltip id={tooltip_id}>
                {props.hint}
            </ReactTooltip>
            <svg
            data-tip
            data-for={tooltip_id}
            onClick={props.onClick}
            onPointerOver={()=> setHover(true)}
            onPointerOut={() => setHover(false)}
            className={ props.className }
            width={props.width}
            height={props.height}
            viewBox={ viewBox }
            fill={ color }
            xmlns="http://www.w3.org/2000/svg">

                { props.children }

            </svg>
        </>
    );
};

export default BaseSvgIcon;