<?php

function getGrades(int $marks)
{
    if ($marks >= 80) {
        return "A";
    } elseif ($marks >= 60) {
        return "B";
    } elseif ($marks >= 40) {
        return "C";
    } else {
        return "F";
    }
}

?>
