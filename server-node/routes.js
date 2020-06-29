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
    console.log(req.body);
    next();
  },
  auth.login
);
api.post(
  `${API_BASE_PATH}${AUTH_BASE_PATH}/register`,
  function checkRegistrationBody(req, res, next) {
    auth.register;
  },
  notImplemented
);
// posts
api.post(`${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`, posts.create);
api.get(
  `${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`,
  posts.search,
  posts.get
);

module.exports = api;
