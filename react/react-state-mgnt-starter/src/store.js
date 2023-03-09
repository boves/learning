import { create } from 'zustand';
const todoStore = (setImmediate) => ({
    // state data and actions will go here  
});
export const useTodosStore = create(todoStore);
