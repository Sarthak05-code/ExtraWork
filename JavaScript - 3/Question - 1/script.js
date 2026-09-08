function calculateTotal(marks) {
  let total = 0;

  for (let i = 0; i < marks.length; i++) {
    total = total + marks[i];
  }

  return total;
}

// Calculate result
function calculateResult() {
  // Get student name
  let name = document.getElementById("name").value;

  // Store marks in an array
  let marks = [
    Number(document.getElementById("m1").value),
    Number(document.getElementById("m2").value),
    Number(document.getElementById("m3").value),
    Number(document.getElementById("m4").value),
    Number(document.getElementById("m5").value),
  ];

  // Calculate total
  let total = calculateTotal(marks);

  // Calculate average
  let average = total / 5;

  // Determine grade
  let grade;

  if (average >= 80) {
    grade = "A";
  } else if (average >= 60) {
    grade = "B";
  } else if (average >= 40) {
    grade = "C";
  } else {
    grade = "F";
  }

  // Display result using DOM
  document.getElementById("result").innerHTML =
    "<h3>Student Result</h3>" +
    "Name: " +
    name +
    "<br>" +
    "Total: " +
    total +
    "<br>" +
    "Average: " +
    average.toFixed(2) +
    "<br>" +
    "Grade: " +
    grade;
}

// Validate marks using onchange
function validateMarks(input) {
  if (input.value < 0 || input.value > 100) {
    alert("Marks must be between 0 and 100");

    input.value = "";
  }
}

// Highlight result using onmouseover
function highlightResult() {
  document.getElementById("result").classList.add("highlight");
}

// Remove highlight
function removeHighlight() {
  document.getElementById("result").classList.remove("highlight");
}
