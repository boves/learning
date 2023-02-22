import { useState } from 'react';
const InputTodo = ({ addTodoItem }) => {
    const [title, setTitle] = useState('');

    const handleChange = (e) => {
        setTitle(e.target.value);
    };
    const handleSubmit = (e) => {
        e.preventDefault();
        console.log(title);
        setTitle('');
        addTodoItem('');
    };
    return(
        <div>
            <form>
                <input 
                    type="text" 
                    placeholder="Add Todo..." 
                    value={title}
                    onChange={handleChange}
                    onSubmit={handleSubmit}
                />
                <button>Submit</button>
            </form>
        </div>
    )    
};
export default InputTodo;
