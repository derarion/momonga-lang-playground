import { Snippet } from "@//types/types";

export const snippets: Snippet[] = [
  {
    key: "helloWorld",
    label: "Hello World",
    code: `print("Hello, World!");\n`,
  },
  {
    key: "arithmeticOperation",
    label: "Arithmetic Operation",
    code: `print(\n  ((2 + 3) * 5 - 7 / 11) % 13\n);\n`,
  },
  {
    key: "variableDeclaration",
    label: "Variable Declaration",
    code: `var x = 1;\nvar y = 2;\n\nprint(x + y);\n`,
  },
  {
    key: "ifStatement",
    label: "If Statement",
    code: `var age = 99; // Enter your age!\nvar price;\n\nif (age <= 3) {\n    price = 100;\n} else if (3 < age && age <= 9) {\n    price = 300;\n} else {\n    price = 500;\n}\n\nprint(price);\n`,
  },
  {
    key: "forStatement",
    label: "For Statement",
    code: `for(var i = 0; i < 10; i = i + 1) {\n    if (i == 3) {\n        continue; // Move on to the next iteration\n    }\n\n    print(i);\n\n    if (i == 7) {\n        break;    // Break out of the loop\n    }\n}\n`,
  },
  {
    key: "array",
    label: "Array",
    code: `var arr = [1, 2, 3];\n\n// Access elements\nprint(arr[0]); // First element\nprint(arr[len(arr) - 1]); // Last element\n\n// Manipulate elements\npush(arr, 4); // Add element to the last\nprint(arr);\n\npop(arr); // Remove the last element\npop(arr);\npop(arr);\npop(arr);\nprint(arr);\n`,
  },
  {
    key: "functionDeclarationAndCall",
    label: "Function Declaration and Call",
    code: `// Declaration\nfunc add(a, b) {\n    return a + b;\n}\n\n// Use () operator to call function\nvar result = add(add(1, 2), 3);\nprint(result);\n`,
  },
  {
    key: "blockScope",
    label: "Block Scope",
    code: `var x = 100;\n\n{\n    var x = 200; // If you remove this line, how would the result change and why?\n    x = 300;\n}\n\nprint(x);\n`,
  },
  {
    key: "recursiveFunction",
    label: "Recursive Function",
    code: `func factorial(n) {\n    print(n);\n    if (n == 0) {\n        return 1;\n    }\n    return n * factorial(n - 1);\n}\nprint(factorial(5));\n`,
  },
  {
    key: "lexicalScoping",
    label: "Lexical Scoping",
    code: `func init() {\n    var name = "Momonga";  // \`name\` is a local variable of init()\n    func printName() {\n        print(name);  // Use variable declared in the parent function\n    }\n    printName();\n}\ninit();
`,
  },
];
