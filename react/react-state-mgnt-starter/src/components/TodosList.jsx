import TodoItem from '@/components/TodoItem';
import { useTodosStore } from '@/store';
const TodosList = ({}) => {
  const todos = useTodosStore((state) => state.todos);
  return (
    <ul>
      {todos.map((todo) => (
        <TodoItem
          key={todo.id}
          itemProp={todo}
          handleChange={handleChange}
          delTodo={delTodo}
          setUpdate={setUpdate}
        />
      ))}
    </ul>
  );
};
export default TodosList;
