# This app allows users to create and manage their own to-do lists.

# Define a function that lets users add items to their to-do list.
def add_item(to_do_list, item):
  to_do_list.append(item)
  print("Item added to your to-do list:", item)

# Define a function that lets users view the items on their to-do list.
def view_list(to_do_list):
  print("Your to-do list:")
  for item in to_do_list:
    print(item)

# Define a function that lets users remove items from their to-do list.
def remove_item(to_do_list, item):
  to_do_list.remove(item)
  print("Item removed from your to-do list:", item)

# Define a function that lets users clear all items from their to-do list.
def clear_list(to_do_list):
  to_do_list.clear()
  print("Your to-do list has been cleared.")

# Define a main function that provides a menu of options for the user to choose from.
def main():
  to_do_list = [] # Initialize an empty to-do list.
  while True:
    print("What would you like to do?")
    print("1. Add an item")
    print("2. View your list")
    print("3. Remove an item")
    print("4. Clear your list")
    print("5. Quit")
    choice = int(input("Enter your choice (1-5): "))

    # Use an if-elif-else statement to execute the appropriate function based on the user's choice.
    if choice == 1:
      item = input("Enter the item you would like to add: ")
      add_item(to_do_list, item)
    elif choice == 2:
      view_list(to_do_list)
    elif choice == 3:
      item = input("Enter the item you would like to remove: ")
      remove_item(to_do_list, item)
    elif choice == 4:
      clear_list(to_do_list)
    elif choice == 5:
      print("Goodbye!")
      break
    else:
      print("Invalid choice. Please try again.")

# Call the main function to run the app.
main()
