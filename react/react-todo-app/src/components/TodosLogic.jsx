import InputTodo from "./InputTodo"
import TodosList from "./TodosList"

const TodosLogic = () => {
    return (
        <div>
                <InputTodo addTodoProps={this.addTodoItem} />
                <TodosList 
                    todos={this.state.todos} 
                    handleChangeProps={this.handleChange}
                    deleteTodoProps={this.delTodo}
                />
        </div>
    )
}
export default TodosLogic
