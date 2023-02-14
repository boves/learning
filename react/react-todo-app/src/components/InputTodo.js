import React, { Component } from "react"

class InputTodo extends Component {
    state = {
        title: ""
    };
    onChange = e => {
        console.log(e.target.value)
        this.setState({
            title: e.target.value
        });
    }
    render() {
        return (
            <form>
                <input 
                    type="text" 
                    placeholder="Add Todo..." 
                    value={this.state.title}
                    onChange={this.onChange}
             />
                <button>Submit</button>
            </form>
        )
    }
}
export default InputTodo
