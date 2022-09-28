import React from 'react';
import { useDispatch } from 'react-redux';

import { NodeProps } from 'Node';
import { changeNodeProperty } from 'nodesSlice';

const LabelNode: React.FC<NodeProps> = (props) => {
    const dispatch = useDispatch();

    const changeFilter: React.ChangeEventHandler<HTMLInputElement> = (event) => {
        dispatch(changeNodeProperty({
            id: props.node['@id'],
            property: "value",
            value: event.target.value
        }));
    };

    return (
        <React.Fragment>
            <div className="prop-row">
                <label htmlFor="text">display value</label>
                <input
                    type="text"
                    value={props.node.node_properties?.value || ""}
                    onChange={changeFilter} />
            </div>
            {props.displayProps?.value && <div className="prop-row prop-result-display">
                = {props.displayProps?.value}
            </div>}
        </React.Fragment>
    );
};

export default LabelNode;