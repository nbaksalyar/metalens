import React from 'react';
import BaseSvgIcon from './BaseSvgIcon';

interface CloseIconProps {
  width?: number;
  height?: number;
  hint?: string;
  fill?: string;
  hoverFill?: string;
  onClick: (event: React.MouseEvent<SVGSVGElement>) => void;
}

const CloseIcon: React.FC<CloseIconProps> = (props) => (
    <BaseSvgIcon
      className='close-icon'
      onClick={props.onClick}
      hint={props.hint}
      width={props.width || 8}
      height={props.height || 8}
      fill={props.fill || "#C8C8C8"}
      hoverFill={props.hoverFill || "#D9BA3E"}>
          <path d="M5.99555 0.344784C6.45237 -0.113175 7.19501 -0.115194 7.6544 0.340357C8.11346 0.795778 8.11544 1.5363 7.65888 1.99387L5.65454 4.00091L7.66099 6.00978C8.11379 6.46409 8.10765 7.19993 7.64727 7.65352C7.18676 8.10693 6.44637 8.10575 5.99371 7.6515L4.00097 5.65651L2.00447 7.65522C1.54765 8.11318 0.805018 8.11519 0.345627 7.65964C-0.113434 7.20422 -0.115478 6.4637 0.341143 6.00613L2.34549 3.99909L0.339098 1.99016C-0.113763 1.53584 -0.107564 0.80001 0.352816 0.346412C0.813327 -0.106925 1.55372 -0.105818 2.00638 0.34843L3.99912 2.34349L5.99555 0.344784Z"/>
    </BaseSvgIcon>
);

export default CloseIcon;
