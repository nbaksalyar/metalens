import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { NodeDisplayProps } from 'displayPropsSlice';
import { RootState, store } from 'store';

export const nodesCounter = { id: 0 };

export enum NodeType {
    Filter = "Filter",
    UProbe = "UProbe",
    Label = "Label"
}

export interface Node {
    "@type": string,
    "@id": number,
    "@inputs": number[],
    node_type: NodeType,
    node_properties: {[property:string]: any},
}

export interface AddNodeAction {
    id: number,
    inputs: number[] | null,
    type: NodeType | null,
    displayProps: NodeDisplayProps | undefined,
}

export interface RemoveNodeAction {
    id: number
}

export interface ChangeNodeTypeAction {
    id: number,
    newNodeType: NodeType,
}

export interface ChangeNodePropAction {
    id: number,
    property: string,
    value: any
}

export interface SetNewState {
    newState: RootState,
}

const initialState: Node[] = [];

export const nodesSlice = createSlice({
    name: 'nodes',
    initialState,
    reducers: {
        addNode: (state, action: PayloadAction<AddNodeAction>) => {
            state.push({
                "@type": "Node",
                "@id": action.payload.id,
                "@inputs": action.payload.inputs || [],
                node_type: action.payload.type || NodeType.Filter,
                node_properties: {},
            });
        },
        removeNode: (state, action: PayloadAction<RemoveNodeAction>) => {
            const nodeById = state.find((n) => n['@id'] === action.payload.id);
            if (nodeById !== undefined) {
                // ...
            }
        },
        changeNodeType: (state, action: PayloadAction<ChangeNodeTypeAction>) => {
            const nodeById = state.find((n) => n['@id'] === action.payload.id);
            if (nodeById !== undefined) {
                nodeById.node_type = action.payload.newNodeType;
            }
        },
        changeNodeProperty: (state, action: PayloadAction<ChangeNodePropAction>) => {
            const nodeById = state.find((n) => n['@id'] === action.payload.id);
            if (nodeById !== undefined) {
                if (action.payload.value === "") {
                    delete nodeById.node_properties[action.payload.property];
                } else {
                    nodeById.node_properties[action.payload.property] = action.payload.value;
                }
            }
        },
        setNewState: (state, action: PayloadAction<SetNewState>) => {
            // TODO
            const ns: any = action.payload.newState;
            // return {...state, ...ns};
        },
    },
});

// Action creators are generated for each case reducer function
export const { addNode, removeNode, changeNodeType, changeNodeProperty, setNewState } = nodesSlice.actions;

export default nodesSlice.reducer;
