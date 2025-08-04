let language = "php";
let todos = {
  php: () => {
    console.log("Salam");
  },
  javascript: () => {},
};
todos[language]();
