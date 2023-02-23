import React from 'react';
import Header from '@/components/Header';
import TodosLogic from './TodosLogic';


const TodoApp = () => {
    return (
      <div className='wrapper'>
        <div>
          <Header />
          <TodosLogic />
        </div>
      </div>
    );
  };
  export default TodoApp;
  