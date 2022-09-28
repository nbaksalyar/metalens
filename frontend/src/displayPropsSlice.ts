import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { addNode } from 'nodesSlice';

export interface NodeDisplayProps {
    // Node starting position.
    defaultX: number,
    defaultY: number,
    // Node value returned from the server.
    value: string,
}

export interface AddNodeAction {
    id: number,
    displayProps: NodeDisplayProps,
}

export interface SetNodeValue {
    id: number,
    value: string
}

export interface SetDefaultPosition {
    id: number,
    defaultX: number,
    defaultY: number,
}

const initialState: { [nodeId: number]: NodeDisplayProps } = {};

export const displayPropsSlice = createSlice({
    name: 'displayProps',
    initialState,
    reducers: {
        setNodeValue: (state, action: PayloadAction<SetNodeValue>) => {
            state[action.payload.id].value = action.payload.value;
            return state;
        }
    },
    extraReducers: (builder) => {
        // TODO: Clean up when a node is removed.
        builder.addCase(
            addNode,
            (state, action) => {
                state[action.payload.id] = {
                    defaultX: action.payload.displayProps?.defaultX || 0,
                    defaultY: action.payload.displayProps?.defaultY || 0,
                    value: ''
                };
            }
        )
    }
});

// Action creators are generated for each case reducer function
export const { setNodeValue } = displayPropsSlice.actions;

export default displayPropsSlice.reducer;
