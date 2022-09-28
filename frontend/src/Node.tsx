import React, { useCallback, useEffect, useRef } from "react";
import Draggable from 'react-draggable';
import { useXarrow } from 'react-xarrows';
import { useDispatch } from 'react-redux';

import { addNode, removeNode, changeNodeType, Node as TNode, nodesCounter, NodeType } from './nodesSlice';
import FilterNode from "FilterNode";
import ProbeNode from "ProbeNode";
import LabelNode from "LabelNode";
import { NodeDisplayProps } from "displayPropsSlice";
import CloseIcon from "UIComponents/Icons/CloseIcon";
import LeftIcon from "UIComponents/Icons/LeftIcon";
import RightIcon from "UIComponents/Icons/RightIcon";
import { NavState } from "interfaces";

export interface NodeProps {
    node: TNode,
    displayProps: NodeDisplayProps | undefined,
}

const Node = React.forwardRef<HTMLDivElement, NodeProps>((props, ref) => {
    const dispatch = useDispatch();
    const updateXarrow = useXarrow();
    const currentPos = useRef({ x: 0, y: 0 });

    const connectNode = useCallback(() => {
        // FIXME: ugly hack
        nodesCounter.id += 1;

        dispatch(addNode({
            id: nodesCounter.id,
            inputs: [props.node["@id"]],
            type: null,
            displayProps: {
                defaultX: currentPos.current.x + 400,
                defaultY: currentPos.current.y + 20,
                value: '',
            }
        }));
    }, [dispatch, props.node, currentPos]);

    const prevNodeType = useCallback(() => {
        // fixme: show a selector for node types or use a props window
        let newNodeType;
        if (props.node.node_type === NodeType.Filter) {
            newNodeType = NodeType.Label;
        } else if (props.node.node_type === NodeType.Label) {
            newNodeType = NodeType.UProbe;
        } else {
            newNodeType = NodeType.Filter;
        };

        dispatch(changeNodeType({ id: props.node["@id"], newNodeType }));
    }, [dispatch, props.node]);

    const nextNodeType = useCallback(() => {
        // fixme: show a selector for node types or use a props window
        let newNodeType;
        if (props.node.node_type === NodeType.Filter) {
            newNodeType = NodeType.UProbe;
        } else if (props.node.node_type === NodeType.UProbe) {
            newNodeType = NodeType.Label;
        } else {
            newNodeType = NodeType.Filter;
        };

        dispatch(changeNodeType({ id: props.node["@id"], newNodeType }));
    }, [dispatch, props.node]);

    const deleteNode = useCallback(() => {
        dispatch(removeNode({ id: props.node["@id"] }));
    }, [dispatch, props.node]);

    /*
    const selectNode: React.MouseEventHandler = (event) => {
        event.stopPropagation();
        dispatch(changeSelection(props.node["@id"]));
    };
    */

    let NodeTypeComponent;
    let title;

    switch (props.node.node_type) {
        case NodeType.Filter:
            NodeTypeComponent = FilterNode;
            title = 'Filter';
            break;
        case NodeType.UProbe:
            NodeTypeComponent = ProbeNode;
            title = 'Probe';
            break;
        case NodeType.Label:
            NodeTypeComponent = LabelNode;
            title = 'Monitor';
            break;
        default:
            // fixme: use node type selector
            NodeTypeComponent = FilterNode;
            break;
    }

    return (
        <Draggable
            onDrag={updateXarrow}
            onStop={(e, dragElement) => {
                currentPos.current = {
                    x: dragElement.x,
                    y: dragElement.y
                };
            }}
            defaultPosition={{
                x: props.displayProps?.defaultX || 0,
                y: props.displayProps?.defaultY || 0
            }}
        >
            <div id={`node${props.node["@id"]}`} className="node" ref={ref}>
                <header className="node-header">
                    <div className="node-title">
                        <LeftIcon onClick={prevNodeType} />
                        <span>
                            {title}
                        </span>
                        <RightIcon onClick={nextNodeType} />
                    </div>
                    <CloseIcon onClick={deleteNode} />
                </header>
                <div className="node-inner">
                    <NodeTypeComponent
                        node={props.node}
                        displayProps={props.displayProps}
                    />
                </div>
                <div className="node-output-handle" onClick={connectNode}></div>
            </div>
        </Draggable>
    );
});

export default Node;
