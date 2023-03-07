import TodoItem from '@/components/TodoItem'
import { useTodosContext } from '@/context/TodosContext';

const TodosList = ({ todosProps, handleChange, delTodo, setUpdate}) => {
    const { todos } = useTodosContext();
    return (
        <ul>
            {todosProps.map((todo) =>
                <TodoItem key={todo.id} itemProp={todo}/>
            )}
        </ul>
    );
};
export default TodosList;
