let api = require("express")();
const API_BASE_PATH = "/api";

const POSTS_BASE_PATH = "/posts";
const AUTH_BASE_PATH = "/auth";

const requestGuards = require("./middleware/requestGuards");

function notImplemented(req, res) {
  res.sendStatus(501);
}

api.post(
  `${API_BASE_PATH}${AUTH_BASE_PATH}/login`,
  requestGuards.expectJsonBody,
  function checkLoginBody(req, res, next) {
    next();
  },
  notImplemented
);
api.post(
  `${API_BASE_PATH}${AUTH_BASE_PATH}/register`,
  function checkRegistrationBody(req, res, next) {
    next();
  },
  notImplemented
);
// posts
api.post(`${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`, notImplemented);
api.get(`${API_BASE_PATH}${POSTS_BASE_PATH}/:userId(\\d+)`, notImplemented);

module.exports = api;
