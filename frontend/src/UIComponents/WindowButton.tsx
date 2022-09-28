import { ButtonType } from "enums";

interface WindowButtonProps {
    type: ButtonType;
    value: string;
    onClick: (event: React.MouseEvent<HTMLButtonElement>) => void;
}

const WindowButton: React.FC<WindowButtonProps> = (props) => {

    let buttonIcon;


    switch(props.type) {
        case ButtonType.OK:
            buttonIcon = (
                <svg width="14" height="11" viewBox="0 0 14 11" fill="#4ADA3E" xmlns="http://www.w3.org/2000/svg">
                    <path d="M0 5.29956L2.39941 5.2695C3.32617 5.7756 4.1556 6.43103 4.86971 7.25984C6.71688 4.35851 8.83911 1.98132 11.1719 0H13C9.73836 3.43249 7.07658 7.10751 4.948 11C3.80965 8.68796 2.21745 6.74171 0 5.29956Z"/>
                </svg>
            );
            break;
        case ButtonType.Cancel:
            buttonIcon = (
                <svg width="11" height="11" viewBox="0 0 11 11" fill="#DABB3E" xmlns="http://www.w3.org/2000/svg">
                    <path d="M6.83179 7.00295C6.46162 6.57254 6.0734 6.13542 5.6667 5.70263C4.33338 6.98694 3.09868 8.43312 2.20515 10.068C1.10902 12.0736 -0.508538 10.4324 0.225366 8.97887C0.815326 7.65279 2.33233 5.81299 4.15571 4.21663C3.15153 3.31383 2.05089 2.49974 0.845404 1.89967C-0.918674 1.02132 0.429038 -0.450609 1.70521 0.133229C2.78028 0.562127 4.26377 1.6664 5.62094 3.03987C6.97681 2.04747 8.40423 1.26974 9.69114 0.965273C9.97044 0.899056 10.7858 0.720096 10.9735 0.987779C11.0442 1.08884 10.9665 1.24053 10.8214 1.40521C10.3275 1.96697 8.99999 2.85766 8.35675 3.37052C7.89248 3.74078 7.42111 4.12769 6.95232 4.53279C7.45076 5.15623 7.89634 5.80174 8.25148 6.44076C8.56021 6.99603 8.8004 7.54654 8.948 8.07368C9.01546 8.31453 9.20044 9.01804 8.97765 9.19159C8.89364 9.25716 8.76044 9.19505 8.61349 9.07517C8.11205 8.6664 7.29671 7.54373 6.83179 7.00295Z" />
                </svg>
            );
            break;
    };

    return (<>
        <button className="window-button" onClick={props.onClick}>
            {buttonIcon}
            {props.value}
        </button>
    </>);
};

export default WindowButton;