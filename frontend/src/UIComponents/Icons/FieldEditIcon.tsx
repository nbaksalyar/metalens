import React from 'react';
import BaseSvgIcon from './BaseSvgIcon';

interface FieldEditIconProps {
  width?: number;
  height?: number;
  hint?: string;
  fill?: string;
  hoverFill?: string;
  onClick: (event: React.MouseEvent<SVGSVGElement>) => void;
}

const FieldEditIcon: React.FC<FieldEditIconProps> = (props) => (
    <BaseSvgIcon
      className='field-edit-icon'
      onClick={props.onClick}
      hint={props.hint}
      width={props.width || 16}
      height={props.height || 16}
      fill={props.fill || "#C8C8C8"}
      hoverFill={props.hoverFill || "#D9BA3E"}>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M8 0C12.418 0 16 3.58203 16 8C16 12.418 12.418 16 8 16C3.58203 16 0 12.418 0 8C0 3.58203 3.58203 0 8 0ZM6.84375 10.9375C6.60677 11.0169 6.36458 11.0898 6.1276 11.168C5.89063 11.2474 5.65365 11.3255 5.41146 11.4049C4.84635 11.5872 4.53776 11.6901 4.47005 11.7083C4.40365 11.7266 4.44531 11.4661 4.58594 10.9193L5.03516 9.20182L8.42188 5.68099L10.2253 7.41667L6.84375 10.9375ZM10.2188 4.40625C10.1341 4.32682 10.0365 4.28516 9.92708 4.29037C9.81771 4.29037 9.72005 4.33333 9.64193 4.41797L8.9987 5.08594L10.8021 6.82813L11.4518 6.14844C11.5312 6.06901 11.5612 5.96615 11.5612 5.85677C11.5612 5.7474 11.5182 5.64453 11.4401 5.57162L10.2188 4.40625Z"/>
    </BaseSvgIcon>
);

export default FieldEditIcon;
