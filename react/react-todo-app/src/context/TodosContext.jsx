import { 
    useState, 
    useEffect,
    createContext,
    useContext,
  } from 'react';

import { v4 as uuidv4 } from 'uuid';

const TodosContext = createContext(null);

export const TodosProvider = ({ children }) => {
    const [todos, setTodos] = useState(getInitialTodos());
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
      id: uuidv4(),
      title: title,
      completed: false,
    };
    setTodos([...todos, newTodo]);
  }; 
  const setUpdate = (updatedTitle, id) => {
    setTodos(
      todos.map((todo) => {
        if (todo.id === id) {
          todo.title = updatedTitle;
        }
        return todo;
      })
    );
  };

  function getInitialTodos() {
    // getting stored items
    const temp = localStorage.getItem('todos');
    const savedTodos = JSON.parse(temp);
    return savedTodos || [];
  }
  useEffect(() => {
    // storing todos items
    const temp = JSON.stringify(todos);
    localStorage.setItem('todos', temp);
  }, [todos]);
    return (
        <TodosContext.Provider 
            value={{
                // 'todos data'
                todos,
                handleChange,
                delTodo,
                addTodoItem,
                setUpdate,
            }}
        >

            {children}
        </TodosContext.Provider>

    );
};
export const useTodosContext = () => useContext(TodosContext);
