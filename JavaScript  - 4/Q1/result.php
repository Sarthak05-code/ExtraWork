<?php

use Odbc\Result;

function calculateResult($marks)
{
    $total = 0;

    foreach ($marks as $mark) {
        $total = $total + $mark;
    }

    $percentage = $total / count($marks);

    return [$total, $percentage];
}

$name = $_POST["name"];
$age = $_POST["age"];

$m1 = $_POST["m1"];
$m2 = $_POST["m2"];
$m3 = $_POST["m3"];

$marks = [$m1, $m2, $m3];

$result = calculateResult($marks);

$total = $result[0];
$percentage = $result[1];

if ($percentage >= 80) {
    $grade = "A";
} elseif ($percentage >= 60) {
    $grade = "B";
} elseif ($percentage > 40) {
    $grade = "C";
} else {
    $grade = "F";
}
?>



<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>Student Result</title>
</head>
<body>
	<h2>Student Result</h2>
	<p><b>Name : </b><?php echo $name; ?></p>
	<p><b>Age : </b><?php echo $age; ?></p>
	<p><b>Total Marks : </b><?php echo $total; ?></p>
	<p><b>Percentage : </b><?php echo $percentage; ?></p>
	<p><b>Grade : </b><?php echo $grade; ?></p>

	<p>
	    <b>Current Date:</b>
		<?php echo date("d-m-y"); ?>
	</p>





</body>
</html>
