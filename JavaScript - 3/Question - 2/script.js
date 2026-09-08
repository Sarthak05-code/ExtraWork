// Array to store tasks
let tasks = [];

// Add task
function addTask(event) {
  // Prevent form from refreshing the page
  event.preventDefault();

  // Get task from input
  let taskText = document.getElementById("taskInput").value;

  // Check if input is empty
  if (taskText == "") {
    alert("Please enter a task");

    return;
  }

  // Add task to array
  tasks.push({
    text: taskText,
    completed: false,
  });

  // Clear input box
  document.getElementById("taskInput").value = "";

  // Display tasks
  displayTasks();
}

// Display tasks
function displayTasks() {
  let list = document.getElementById("taskList");

  // Clear previous list
  list.innerHTML = "";

  // Loop through tasks
  for (let i = 0; i < tasks.length; i++) {
    // Create list item
    let li = document.createElement("li");

    // Display task text
    li.innerHTML = tasks[i].text;

    // Click task to mark completed
    li.onclick = function () {
      tasks[i].completed = !tasks[i].completed;

      displayTasks();
    };

    // If completed, add CSS class
    if (tasks[i].completed) {
      li.classList.add("completed");
    }

    // Create Delete button
    let deleteButton = document.createElement("button");

    deleteButton.innerHTML = "Delete";

    // Delete task
    deleteButton.onclick = function (event) {
      // Prevent li onclick
      event.stopPropagation();

      // Remove task from array
      tasks.splice(i, 1);

      // Display updated list
      displayTasks();
    };

    // Add delete button to li
    li.appendChild(deleteButton);

    // Add li to ul
    list.appendChild(li);
  }

  // Update task count
  updateCount();
}

// Count pending and completed tasks
function updateCount() {
  let pending = 0;
  let completed = 0;

  for (let i = 0; i < tasks.length; i++) {
    if (tasks[i].completed) {
      completed++;
    } else {
      pending++;
    }
  }

  // Display count
  document.getElementById("count").innerHTML =
    "Pending: " + pending + " | Completed: " + completed;
}
