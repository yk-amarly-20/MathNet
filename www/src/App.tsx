import React from "react";
import * as wasm from "math-net";
import { BrowserRouter, Switch, Route, Router } from 'react-router-dom';
import Header from "./components/Layouts/Head";
import PopularCard from "./components/Popular";

const App: React.FC = () => {
  return (
    <div>
      <header>
        <Header title="MathNet" />
      </header>
      <body>
        <PopularCard />
      </body>
    </div>
  );
};

export default App;
