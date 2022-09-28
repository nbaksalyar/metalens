import React from 'react';
import BaseSvgIcon from './BaseSvgIcon';

interface RightIconProps {
  width?: number;
  height?: number;
  hint?: string;
  fill?: string;
  hoverFill?: string;
  onClick: (event: React.MouseEvent<SVGSVGElement>) => void;
}

const RightIcon: React.FC<RightIconProps> = (props) => (
    <BaseSvgIcon
      className='left-icon'
      onClick={props.onClick}
      hint={props.hint}
      width={props.width || 6}
      height={props.height || 10}
      fill={props.fill || "#C8C8C8"}
      hoverFill={props.hoverFill || "#D9BA3E"}>
        <path d="M0.23867 1.38555C-0.0782792 1.06968 -0.0798413 0.556134 0.235742 0.238903C0.55113 -0.0783286 1.06422 -0.0798907 1.38117 0.235777L5.57724 4.42523C5.89419 4.74089 5.89575 5.25444 5.58016 5.57167L1.38117 9.76405C1.06422 10.0799 0.551129 10.0784 0.235741 9.76112C-0.079842 9.44389 -0.0782799 8.93015 0.23867 8.61448L3.859 4.99991L0.23867 1.38555Z" />
    </BaseSvgIcon>
);

export default RightIcon;
