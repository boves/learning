import InputTodo from "@/components/InputTodo";
import TodosList from "@/components/TodosList";
import { useState } from 'react'

const TodosLogic = () => {
    const [todos, setTodos] = useState([
        {
          id: 1,
          title: 'Setup development environment',
          completed: true,
        },
        {
          id: 2,
          title: 'Develop website and add content',
          completed: false,
        },
        {
          id: 3,
          title: 'Deploy to live server',
          completed: false,
        },
      ]);
      const handleChange = (id) => {
        console.log('clicked', id);
        setTodos((prevState) => 
            prevState.map((todo) => {
                if (todo.id === id) {
                    return {
                        ...todo,
                        completed: !todo.completed,
                    };
                }
                return todo;
            })
        );
    };
    const delTodo =(id) => {
      console.log('deleted', id);
      setTodos([
        ...todos.filter((todo) => {
          return todo.id !== id;
        }),
      ]);
    };
    const addTodoItem = (title) => {
      const newTodo = {
        id: 4,
        title: title,
        completed: false,
      };
      setTodos([...todos, newTodo]);
    }; 

    return (
      <div>
        <InputTodo />
        <TodosList 
          todosProps={todos} 
          setTodos={setTodos} 
          handleChange={handleChange}
          delTodo={delTodo}
          addTodoItem={addTodoItem}
        />
      </div>
    )
  }
  export default TodosLogic;
  