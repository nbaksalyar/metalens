// Handles UI state.

import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface UiState {
    // Node ID, null if nothing is selected
    selectedNode: number | null,
    // Shows bpf assembly code
    showGeneratedCode: boolean,
    // Generated assembly code
    asmCode: string,
}

const initialState: UiState = {
    selectedNode: null,
    showGeneratedCode: false,
    asmCode: '',
};

export const uiSlice = createSlice({
    name: 'ui',
    initialState,
    reducers: {
        changeSelection: (state, action: PayloadAction<number | null>) => {
            state.selectedNode = action.payload;
        },
        changeShowGeneratedCode: (state, action: PayloadAction<boolean>) => {
            state.showGeneratedCode = action.payload;
        },
        setAsmCode: (state, action: PayloadAction<string>) => {
            state.asmCode = action.payload;
        }
    },
});

// Action creators are generated for each case reducer function
export const { changeSelection, changeShowGeneratedCode, setAsmCode } = uiSlice.actions;

export default uiSlice.reducer;
