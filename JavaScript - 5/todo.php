<?php
$file = "tasks.txt";

if ($_SERVER["REQUEST_METHOD"] == "POST") {
    $task = $_POST["task"];

    $handle = fopen($file, "a");

    fwrite($handle, $task . "\n");

    fclose($handle);
}
?>

<!DOCTYPE html> <html> <head> <title>My To-Do List</title> <style> body { font-family: Arial; margin: 30px; } h2 { color: darkblue; } li { margin: 10px; padding: 8px; border: 1px solid gray; width: 300px; } a { display: inline-block; margin-top: 20px; } </style> </head> <body> <h2>My To-Do List</h2> <ul> <?php
// Open file for reading $handle = fopen($file, "r"); // Read file line by line while (($line = fgets($handle)) !== false) { // Remove extra whitespace $task = trim($line); // Display task echo "<li>" . htmlspecialchars($task) . "</li>"; } // Close the file fclose($handle);
?> </ul> <a href="index.html"> Add Another Task </a> </body> </html>
