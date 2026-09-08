<?php
include "functions.php";

$name = $_POST["name"];
$age = $_POST["age"];
$marks = $_POST["marks"];

$students = [
    [
        "name" => $name,
        "age" => $age,
        "marks" => $marks,
        "grade" => getGrades($marks),
    ],
    ["name" => "Ram", "age" => 20, "marks" => 85, "grade" => getGrades(85)],
    ["name" => "Sita", "age" => 21, "marks" => 72, "grade" => getGrades(72)],
];
?>



<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>Student Information System</title>
</head>
<body>
    <h2>Student Information System</h2>

    <table border="1" cellpadding = "10">
        <tr>
            <th>Name</th>
            <th>Age</th>
            <th>Marks</th>
            <th>Grades</th>
        </tr>
        <?php foreach ($students as $student) {
            echo "<tr>";
            echo "<td>" . $student["name"] . "</td>";
            echo "<td>" . $student["age"] . "</td>";
            echo "<td>" . $student["marks"] . "</td>";
            echo "<td>" . $student["grade"] . "</td>";
            echo "</tr>";
        } ?>
    </table>

</body>
</html>
