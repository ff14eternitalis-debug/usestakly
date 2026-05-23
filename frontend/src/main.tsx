import React from "react";
import ReactDOM from "react-dom/client";

import { AppProviders } from "./app/providers";
import { installUmamiScript } from "./lib/analytics";

import "./styles.css";

installUmamiScript();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AppProviders />
  </React.StrictMode>
);
