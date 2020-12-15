import React from "react";
import * as wasm from "math-net";
import { BrowserRouter, Switch, Route, Router } from 'react-router-dom';
import Header from "./components/Layouts/Head";

const App: React.FC = () => {
  wasm.greet();
  return (
    <header>
      <Header title="MathNet" />
    </header>
  );
};

export default App;
