import React from "react";

import Auth from "./Components/Auth/Auth";
import Dashboard from "./Components/Dashboard/Dashboard";
import Form from "./Components/Form/Form";
import Post from "./Components/Post/Post";
import { Switch, Route, HashRouter } from "react-router-dom";
function Routes(props) {
  return (
    <HashRouter>
      <Switch>
        <Route path="/" exact component={Auth} />
        <Dashboard />
        <Post />
      </Switch>
    </HashRouter>
  );
}

export default Routes;
