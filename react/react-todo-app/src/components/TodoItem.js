import React from "react"

const TodoItem = (props) => {
    return (
        <li>
            <input 
                type="checkbox" 
                checked={props.todo.completed}
                onChange={() => props.handleChangeProps() } // console.log("clicked")}
            /> {props.todo.title}
        </li>
    )
}

export default TodoItem
