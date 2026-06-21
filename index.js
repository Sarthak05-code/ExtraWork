function main() {
  // this is a comment
  /*
        this is a 
        multi line comment
    */

  var a = 15,
  b = 10;
  add = a + b;
  sub = a - b;
  product = a * b;
  div = a / b;
  mod = a % b;
  console.log("The sum is: " + add);
  console.log("The difference is: " + sub);
  console.log("The product is: " + product);
  console.log("The division is : " + div);
  console.log("The modulus is: " + mod);

  // assignment operator
  ((a = 20), (b = 11));
  a *= b;

  a -= b;

  a += b;

  a /= b;

  a %= b;
  console.log("The new difference is: " + a);
  console.log("The new product is: " + a);
  console.log("The new sum is : " + a);

  console.log("The new mod is: " + a);

  console.log("The new divide is :  " + a);

  let avalue = 25,
    bvalue = 90,
    cvalue = 100;

  if (avalue >= bvalue) console.log("A is greater: ");
  else if (cvalue <= avalue) console.log("c is greater");
  else console.log("B is greater");

  name = "15";
  age = 15;

  if (name === age) console.log("They are the same ");
  else console.log("They arent equal");

  if (name != age) console.log("this is true");
  else console.log("This is false");

  let x = 10,
    y = 10,
    z = 10;

  if (x == y && y == z) console.log("They have the same value: ");
  else console.log("They dont have the same value");

  var value = -10;

  if (value > 0) console.log("This is a positive number");
  else if (value < 0) console.log("this is a negative number");
  else console.log("This is a zero");

  var marks = 39;

  if (marks > 80) console.log("Distinction");
  else if (marks > 60 && marks <= 79) console.log("First Division");
  else if (marks > 40 && marks <= 59) console.log("Second Division");
  else console.log("Fail");

  var choice = "-";

  switch (choice) {
    case "+":
      console.log("You picked addition");
      break;
    case "-":
      console.log("You picked substraction");
      break;
    case "*":
      console.log("You picked production");
      break;

    case "/":
      console.log("You picked division");
      break;

    default:
      console.log("You picked the wrong choice");
      break;
  }

  for (var i = 0; i < 10; ++i) {
    console.log(i + 1);
  }

  var i = 0;
  while (i < 10) {
    console.log(i + 1);
    i++;
  }

  var i = 0;

  do {
    console.log(i + 1);
    i++;
  } while (i < 10);

  console.log("\n");

  for (var i = 1; i <= 10; ++i) {
    console.log("2 x ", i, " = ", 2 * i);
  }

  var result = 1;

  for (var i = 1; i <= 5; i++) {
    result *= i;
  }

  console.log("The factorial of 5 is : ", result);
}

main();