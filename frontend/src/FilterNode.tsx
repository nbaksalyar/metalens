import React from 'react';
import { useDispatch } from 'react-redux';

import { NodeProps } from 'Node';
import { changeNodeProperty } from 'nodesSlice';
import FunctionalField from 'UIComponents/FunctionalField';
import { FunctionalFieldMode } from 'enums';

const FilterNode: React.FC<NodeProps> = (props) => {
    const dispatch = useDispatch();

    const changeFilter: React.ChangeEventHandler<HTMLInputElement> = (event) => {
        dispatch(changeNodeProperty({
            id: props.node['@id'],
            property: "value",
            value: event.target.value
        }));
    };

    return (
        <div className="prop-row">
            <label htmlFor="text">filter</label>
            <input
                type="text"
                value={props.node.node_properties?.value || ""}
                onChange={changeFilter} />
            {/*<FunctionalField
                defaultFieldValue=''
                fieldValue=''
                defaultLabelValue=''
                labelValue=''
                placeHolder=''
                mode={FunctionalFieldMode.Functional} />*/}
        </div>
    );
};

export default FilterNode;