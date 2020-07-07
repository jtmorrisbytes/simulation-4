import React from "react";
import { Link } from "react-router-dom";
import "./Auth.scss";
function handleInputChange(inputEvent) {}
function handleLogin(e) {
  e.preventDefault();
  console.log(
    "handle login",
    e.target.elements.username.value,
    e.target.elements.password.value
  );
  e.target.elements.username.value = "";
}

function Auth(props) {
  // let [username, updateUsername] = React.useState("");
  // let [password, updatePassword] = React.useState("");
  return (
    <div className="Auth">
      <img src={"/helo logo.svg"} />
      <h1>Helo</h1>

      <form onSubmit={handleLogin}>
        <div>
          <label htmlFor="#username">Username: </label>
          <input
            // value={username}
            placeholder={"Username"}
            // onChange={function (e) {
            //   updateUsername(e.target.value);
            // }}
            id="username"
            name="username"
          />
        </div>
        <div>
          <label for="#password">Password: </label>
          <input
            id="password"
            type="password"
            // value={password}
            // onChange={function (e) {
            //   updatePassword(e.target.value);
            // }}
            name="password"
          />
        </div>
        <div>
          <button type="submit">Login</button>
          <Link to="/register" type="button">
            Register
          </Link>
        </div>
      </form>
    </div>
  );
}

export default Auth;
