import { FunctionalFieldMode } from "enums";
import FieldDeleteIcon from "./Icons/FieldDeleteIcon";
import FieldEditIcon from "./Icons/FieldEditIcon";

// FIXME: probably there is a better name for this component
interface FunctionalFieldProps {
    fieldValue: string;
    labelValue: string;
    defaultFieldValue?: string;
    defaultLabelValue?: string;
    placeHolder?: string;
    editable?: boolean;
    deletable?: boolean;
    mode: FunctionalFieldMode;
}

const FunctionalField: React.FC<FunctionalFieldProps> = (props) => {

    const changeField: React.ChangeEventHandler<HTMLInputElement> = (event) => {

    };

    const changeLabel: React.ChangeEventHandler<HTMLInputElement> = (event) => {

    };

    const editFieldHandler = (event: React.MouseEvent<SVGElement>) => {
        console.log("Edit");
    };

    const deleteFieldHandler = (event: React.MouseEvent<SVGElement>) => {
        console.log("Delete");
    };

    return (
        <div className="functional-input-group">
            <label htmlFor="text">{ props.labelValue || props.defaultLabelValue || "Default Label" }</label>
            <input
                readOnly
                placeholder={ props.placeHolder || "" }
                type="text"
                value={ props.fieldValue || props.defaultFieldValue }
            />
            {props.editable ?
                (<FieldEditIcon hint="Edit Field" onClick={editFieldHandler} />) : <></>}
            {props.deletable ?
                (<FieldDeleteIcon hint="Delete Field" onClick={deleteFieldHandler} />) : <></>}
        </div>
    );
};

FunctionalField.defaultProps = {
    editable: true,
    deletable: true,
    defaultFieldValue: "",
    defaultLabelValue: "Default Label",
};

export default FunctionalField;