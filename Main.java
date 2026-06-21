import java.util.List;
import java.util.stream.*;

public class Main {

    public static void main(String[] args) {
        List<Student> students = List.of(
            new Student("Sarthak", 76),
            new Student("Harry", 10),
            new Student("Sagar", 11),
            new Student("Kumar", 55)
        );

        final List<String> toppers = students.stream()
                    .filter(s -> s.marks >= 50)
                    .sorted((a , b) -> b.marks - a.marks)
                    .map(s -> s.name)
                    .collect(Collectors.toList());

        System.out.println(toppers);
    }
}

class Student {
    String name;
    int marks;

    Student(String name , int marks) {
        this.name = name;
        this.marks = marks;
    }
}