public class Main {
    public static void main(String[] args) {
        Employee[] staffs = {
            new Manager("Alice", 50000, 5),
            new Developer("Sarthak", 125000, 12)
        };
        for (Employee a : staffs) {
            a.printpaySlip();
        }
    }
}

abstract class Employee {
    protected String name;
    protected double baseSalary;

    public Employee(String name , double baseSalary) {
        this.name = name;
        this.baseSalary = baseSalary;
    }

    public void printpaySlip() {
        System.out.println("Employee: " + name);
        System.out.println("Salary : " + baseSalary);
        System.out.println("Bonus: " + calculateBonus());
        System.out.println("Total: " + (baseSalary +  calculateBonus()));
    }

    public abstract double calculateBonus();
}

class Manager extends Employee {
    private int teamSize;
    public Manager(String name , double baseSalary , int teamSize) {
        super(name, baseSalary);
        this.teamSize = teamSize;
    }

    @Override
    public double calculateBonus() {
        return teamSize * 500;
    }
}

class Developer extends Employee {
    private int projectCompleted;
    public Developer(String name , double baseSalary , int projectCompleted) {
        super(name, baseSalary);
        this.projectCompleted = projectCompleted;
    }

    @Override
    public double calculateBonus() {
        return projectCompleted * 1000;
    }
}