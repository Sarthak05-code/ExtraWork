public class Main {

    public static void main(String[] args) {
        Caller caller  = new Caller();
        caller.runner();
        caller.Text();
    }
}


interface Name{
    public void runner();
}

abstract class Mainer {
    public abstract void Text();
}

class Caller extends Mainer implements Name {
    @Override
    public void runner() {
        System.out.println("This should work as intended. ");
    }

    @Override
    public void Text() {
        System.out.println("This should also work. ");
    }
}