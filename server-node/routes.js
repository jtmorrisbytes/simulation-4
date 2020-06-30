const express = require("express");
let api = express();
const API_BASE_PATH = "/api";

const POSTS_BASE_PATH = "/posts";
const AUTH_BASE_PATH = "/auth";

const requestGuards = require("./middleware/requestGuards");
const posts = require("./controllers/post");
const auth = require("./controllers/auth");

function notImplemented(req, res) {
  res.sendStatus(501);
}

api.use(express.json());
api.post(
  `${API_BASE_PATH}${AUTH_BASE_PATH}/login`,
  // requestGuards.expectJsonBody,
  function checkLoginBody(req, res, next) {
    if (typeof req.body.username !== "string") {
      res
        .status(422)
        .send(
          "Missing or invalid type for username, recieved: " +
            typeof req.body.username
        );
    } else if (req.body.username.length === 0) {
      res.status(422).send("Username cannot be empty");
    } else if (typeof req.body.password !== "string") {
      res
        .status(422)
        .send(
          "Missing or invalid type for password, recieved: " +
            typeof req.body.password
        );
    } else if (req.body.password.length === 0) {
      res.status(422).send("Password cannot be empty");
    } else {
      next();
    }
  },
  auth.login
);
api.post(
  `${API_BASE_PATH}${AUTH_BASE_PATH}/register`,
  function checkRegistrationBody(req, res, next) {
    if (
      typeof req.body.username === "string" &&
      typeof req.body.password === "string" &&
      typeof req.body.profile === "string"
    ) {
      next();
    } else {
      res.status(422);
      if (req.body.username == null) {
        res.send("Missing username in request");
      } else if (req.body.password == null) {
        res.send("Missing username in request");
      } else if (req.body.profile == null) {
        res.send("Missing profile in request");
      }
    }
  },
  auth.register
);
// posts
api.post(`${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`, posts.create);
api.get(
  `${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`,
  posts.search,
  posts.get
);

module.exports = api;
