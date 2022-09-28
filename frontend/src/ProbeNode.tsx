import React from 'react';
import { useDispatch } from 'react-redux';

import { NodeProps } from 'Node';
import { changeNodeProperty } from 'nodesSlice';

const ProbeNode: React.FC<NodeProps> = (props) => {
    const dispatch = useDispatch();

    const changeProperty = (property: string): React.ChangeEventHandler<HTMLInputElement> => ((event) => {
        dispatch(changeNodeProperty({
            id: props.node['@id'],
            property,
            value: event.target.value
        }));
    });

    return (
        <React.Fragment>
            <div className="prop-row">
                <label htmlFor="text">program</label>
                <input type="text"
                    value={props.node.node_properties?.program || ""}
                    onChange={changeProperty('program')} />
            </div>
            <div className="prop-row">
                <label htmlFor="text">function</label>
                <input type="text"
                    value={props.node.node_properties?.function || ""}
                    onChange={changeProperty('function')} />
            </div>
            <div className="prop-row">
                <label htmlFor="text">probe</label>
                <input type="text"
                    value={props.node.node_properties?.probe || ""}
                    onChange={changeProperty('probe')} />
            </div>
        </React.Fragment>
    );
};

export default ProbeNode;