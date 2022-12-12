# Prompt 
Write a React.JS app in typescript that allows the user to write a new journal entry every day. If they write 200 words, then they can earn points. The system keeps track of how many days in a row they have written and tells the user what their current streak is. If the user writes 5 days in a row, they get a 50 point bonus.

# Summary from ChatGPT
Here is an example of a simple journaling app built with React and TypeScript:

[...]

In this example, the Journal component keeps track of the user's journal entries, their current streak, and their points in its state. The handleChange and handleSubmit methods update the state when the user writes a new entry or submits the form. The render method renders the form and displays the current streak and points.

When the user writes an entry that is at least 200 words long, the handleSubmit method increments the points by 1. If the user has written for 4 days in a row, it also increments the points by 50.

This is just one possible implementation of a journaling app in React and TypeScript. There are many other ways to design and implement such an app.