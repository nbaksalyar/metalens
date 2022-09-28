import { configureStore } from '@reduxjs/toolkit'
import displayPropsSlice from 'displayPropsSlice';
import nodesReducer from './nodesSlice';
import uiReducer from './uiSlice';

export const store = configureStore({
    reducer: {
        nodes: nodesReducer,
        ui: uiReducer,
        displayProps: displayPropsSlice,
    },
})

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;

// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;
