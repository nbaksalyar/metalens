import { RootState, store } from "./store";
import CloseIcon from "UIComponents/Icons/CloseIcon";
import WindowButton from "UIComponents/WindowButton";
import { ButtonType } from "enums";
import { EditorLanguage } from "enums";
import { useDispatch } from "react-redux";
import { setNewState } from "./nodesSlice";
import CodeMirror from "UIComponents/CodeMirror";

interface ExportImportProps {
    isVisible: boolean;
    windowTitle: string;
    state: RootState;
    closeExportImportModal: Function;
}

const ExportImportWindow: React.FC<ExportImportProps> = (props) => {

    const dispatch = useDispatch();

    if (!props.isVisible) {
        return null;
    };

    const updateState = () => {
        // FIXME: Dirty hack for now, will be fixed later
        const textAreaValue =
            document.querySelector(".window-inner textarea")?.textContent;
        const nextState = JSON.parse(textAreaValue!);
        dispatch(setNewState({newState: nextState}));
    };

    return (
        <div className="export-import-window">
            <header className="node-header">
                <div className="node-title">
                    <span>
                        Export/Import Program
                    </span>
                </div>
                <CloseIcon onClick={props.closeExportImportModal()} />
            </header>
            <div className="window-inner">
                <label>Current Program (JSON):</label>
                <CodeMirror
                    mode={EditorLanguage.JSON}
                    defaultValue={JSON.stringify( props.state, null, 2 ) || ""}
                />
            </div>
            <div className="window-buttons-area">
                <WindowButton
                    type={ ButtonType.Cancel }
                    value="Dismiss"
                    onClick={props.closeExportImportModal()} />
                <WindowButton
                    type={ ButtonType.OK }
                    value="Update"
                    onClick={updateState} />
            </div>
        </div>
    );
};

export default ExportImportWindow;
