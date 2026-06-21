public class Main {

    public static void main(String[] args) {
        Caller caller  = new Caller();
        caller.runner();
    }
}


interface Name{
    public void runner();
}


class Caller implements Name {
    @Override
    public void runner() {
        System.out.println("This should work as intended. ");
    }
}