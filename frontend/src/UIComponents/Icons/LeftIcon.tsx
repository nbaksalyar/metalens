import React from 'react';
import BaseSvgIcon from './BaseSvgIcon';

interface LeftIconProps {
  width?: number;
  height?: number;
  hint?: string;
  fill?: string;
  hoverFill?: string;
  onClick: (event: React.MouseEvent<SVGSVGElement>) => void;
}

const LeftIcon: React.FC<LeftIconProps> = (props) => (
    <BaseSvgIcon
      className='left-icon'
      onClick={props.onClick}
      hint={props.hint}
      width={props.width || 6}
      height={props.height || 10}
      fill={props.fill || "#C8C8C8"}
      hoverFill={props.hoverFill || "#D9BA3E"}>
        <path d="M5.57723 8.61445C5.89418 8.93032 5.89574 9.44387 5.58016 9.7611C5.26477 10.0783 4.75168 10.0799 4.43473 9.76422L0.238666 5.57477C-0.0782829 5.25911 -0.0798442 4.74556 0.235739 4.42833L4.43473 0.235949C4.75168 -0.0799153 5.26477 -0.0783526 5.58016 0.238879C5.89574 0.55611 5.89418 1.06985 5.57723 1.38552L1.95691 5.00009L5.57723 8.61445Z"/>
    </BaseSvgIcon>
);

export default LeftIcon;
