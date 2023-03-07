import styles from '@/styles/TodoItem.module.css';
import { useState, useRef } from 'react';
import { FaTrash } from "react-icons/fa";
import { AiFillEdit } from "react-icons/ai";
import { useTodosContext } from '@/context/TodosContext';

const TodoItem = ({ itemProp, handleChange, delTodo, setUpdate }) => {
    const { handleChange, delTodo, setUpdate } = useTodosContext();
    const editInputRef = useRef(null);
    const [editing, setEditing] = useState(false);
    const completedStyle = {
        fontStyle: 'italic',
        color: '#595959',
        opacity: 0.4,
        textDecoration: 'line-through',
      };
    const handleEditing = () => {
        setEditing(true);
    };
    let viewMode = {};
    let editMode = {};
    if (editing) {
        viewMode.display = 'none';
    } else {
        editMode.display = 'none'
    }
    const handleUpdatedDone = (event) => {
        if (event.key === 'Enter') {
            setUpdate(editInputRef.current.value, itemProp.id)
            setEditing(false);
        }
    };
    return (
        <li className={styles.item}>
            <div className={styles.content} style={viewMode}>
                <input 
                    type="checkbox" 
                    checked={itemProp.completed}
                    onChange={() => handleChange(itemProp.id)}
                />
                <button onClick={handleEditing}>
                    <AiFillEdit />
                </button>
                <button onClick={() => delTodo(itemProp.id)}>
                    <FaTrash />
                </button>
                <span style={itemProp.completed ? completedStyle: null}>
                    {itemProp.title}
                </span>
            </div>
            <input
                type="text"
                ref={editInputRef}
                defaultValue={itemProp.title}
                className={styles.textInput}
                style={editMode}
                onKeyDown={handleUpdatedDone}
            />
        </li>
    );
};
export default TodoItem;
